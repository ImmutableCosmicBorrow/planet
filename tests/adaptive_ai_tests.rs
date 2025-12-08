mod common;

use common_game::components::asteroid::Asteroid;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator};
use common::*;
use planet::{create_planet, Ai};


const ROCKET_COEFFICIENT : f32 = 0.4;
const BASIC_GEN_COEFFICIENT : f32 = 0.8;
const COMPLEX_GEN_COEFFICIENT : f32 = 0.8;
const SUNRAY_PROBABILITY : f32 = 0.8;
const EXPLORER_REQUEST_PROBABILITY : f32 = 0.2;






// Ignored because now my dummy explorer always expects a response, but if ai decides to not fulfil the request, planet will not respond and test will fail
// will remove the ignore when we decide what to do with that None
#[test]
#[ignore]
fn test_adaptive_ai(){
    // Variables for statistics
    let mut accepted_explorer_requests = 0;
    let mut refused_explorer_requests = 0;
    let mut asteroids_avoided = 0;
    let mut sunrays_received = 0;
    let mut iterations = 0;

    let mut sunray_probability_modifier = 0.0;


    // Creating the Planet

    let planet_ai = Ai::new(false, ROCKET_COEFFICIENT, BASIC_GEN_COEFFICIENT, COMPLEX_GEN_COEFFICIENT );
    // Channel 1: Orchestrator -> Planet
    let (tx_orchestrator, rx_orchestrator_to_planet) = crossbeam_channel::bounded::<OrchestratorToPlanet>(1);
    // Channel 2: Planet -> Orchestrator
    let (tx_planet_to_orchestrator, rx_orchestrator) =crossbeam_channel::bounded::<PlanetToOrchestrator>(1);

    // Channel 3: Explorer -> Planet
    let (tx_explorer, rx_explorer_to_planet) = crossbeam_channel::bounded::<ExplorerToPlanet>(1);
    // Channel 4: Planet -> Explorer
    let (_tx_planet_to_explorer, _rx_planet_to_explorer) = crossbeam_channel::bounded::<PlanetToExplorer>(1);

    let planet = create_planet(
        planet_ai,
        vec![BasicResourceType::Carbon],
        vec![ComplexResourceType::AIPartner, ComplexResourceType::Diamond, ComplexResourceType::Dolphin, ComplexResourceType::Life, ComplexResourceType::Robot, ComplexResourceType::Water],
        (rx_orchestrator_to_planet, tx_planet_to_orchestrator),
        rx_explorer_to_planet,
    ).expect("Planet creation failed");

    // Channels for dummy explorer
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::bounded::<PlanetToExplorer>(1);

    // Use tx_orchestrator and rx_orchestrator for Planet <-> Orchestrator communications
    // Use tx_explorer and rx_explorer for Planet <-> Explorer communications

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the Planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the Planet that an explorer arrived
    let _ = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::IncomingExplorerRequest {explorer_id : 0, new_mpsc_sender : tx_to_explorer});


    // 4. Loop
    print!("Asteroids (A) and Sunrays (S) sequence : ");
    loop {
        iterations += 1;

        // a. Orchestrator sends a Sunray with probability SUNRAY_PROBABILITY + modifier, otherwise sends Asteroid
        if rand::random::<f32>() <= SUNRAY_PROBABILITY + sunray_probability_modifier {
            print!("S");
            let _ = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));
            sunrays_received += 1;
        } else {
            print!("A");
            let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Asteroid(Asteroid::default()));
            match response {
                PlanetToOrchestrator::AsteroidAck {rocket : None, .. } => break,

                PlanetToOrchestrator::AsteroidAck {rocket : Some(rocket), .. } => asteroids_avoided += 1,

                _ => panic!("Expected AsteroidAck but received different response")
            }
        }

        // b. Explorer asks Planet to create Carbon with probability EXPLORER_REQUEST_PROBABILITY, otherwise does nothing
        if rand::random::<f32>() <= EXPLORER_REQUEST_PROBABILITY {
            let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest {explorer_id : 0, resource : BasicResourceType::Carbon});
            match response {
                PlanetToExplorer::GenerateResourceResponse { resource : None } => refused_explorer_requests += 1,
                PlanetToExplorer::GenerateResourceResponse { resource : Some(_) } => accepted_explorer_requests += 1,
                _ => panic!("Expected GenerateResourceResponse but received different response")
            }
        }
    }

    println!("\nPlanet is destroyed, concluding test.");
    println!("Iterations : {}", iterations);
    println!("Accepted explorer's requests: {} - {}%", accepted_explorer_requests, ((accepted_explorer_requests as f32 / (refused_explorer_requests as f32 + accepted_explorer_requests as f32))*100.0) as u32);
    println!("Refused explorer's requests: {} - {}%", refused_explorer_requests, ((refused_explorer_requests as f32 / (refused_explorer_requests as f32 + accepted_explorer_requests as f32))*100.0) as u32);
    println!("Asteroids avoided: {}", asteroids_avoided);
    println!("Sunrays received: {} - {}% of Sunrays + Asteroids", sunrays_received, ((sunrays_received as f32 / (sunrays_received as f32 + asteroids_avoided as f32 + 1.0))*100.0) as u32);

    // 5. Orchestrator stops the Planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);

    // 6. Stop thread
    drop(tx_orchestrator);
    let _ = handle.join();
}