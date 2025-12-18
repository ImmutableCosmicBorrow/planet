#![allow(clippy::pedantic)]

mod common;

use common::*;
use common_game::components::forge::Forge;
use common_game::components::resource::BasicResourceType;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use planet::{Ai, create_planet};
use std::thread;
use std::time::Duration;

const BASIC_GEN_COEFFICIENT: f32 = 0.9;
const COMPLEX_GEN_COEFFICIENT: f32 = 0.9;
const SUNRAY_PROBABILITY: f32 = 0.8;
const EXPLORER_REQUEST_PROBABILITY: f32 = 0.4;

#[test]
fn test_adaptive_ai() {
    // Variables for statistics
    let mut accepted_explorer_requests = 0;
    let mut refused_explorer_requests = 0;
    let mut dont_have_energycell = 0;
    let mut asteroids_avoided = 0;
    let mut sunrays_received = 0;
    let mut iterations = 0;

    let mut sunray_probability_modifier = 0.0;

    let forge = Forge::new().unwrap();

    // Creating the Planet

    let planet_ai = Ai::new(
        false,
        BASIC_GEN_COEFFICIENT,
        COMPLEX_GEN_COEFFICIENT,
        Duration::from_secs(1),
        Duration::from_millis(100),
    );
    // Channel 1: Orchestrator -> Planet
    let (tx_orchestrator, rx_orchestrator_to_planet) =
        crossbeam_channel::bounded::<OrchestratorToPlanet>(1);
    // Channel 2: Planet -> Orchestrator
    let (tx_planet_to_orchestrator, rx_orchestrator) =
        crossbeam_channel::bounded::<PlanetToOrchestrator>(1);

    // Channel 3: Explorer -> Planet
    let (tx_explorer, rx_explorer_to_planet) = crossbeam_channel::bounded::<ExplorerToPlanet>(1);
    // Channel 4: Planet -> Explorer
    let (_tx_planet_to_explorer, _rx_planet_to_explorer) =
        crossbeam_channel::bounded::<PlanetToExplorer>(1);

    let planet = create_planet(
        planet_ai,
        (rx_orchestrator_to_planet, tx_planet_to_orchestrator),
        rx_explorer_to_planet,
    )
    .expect("Planet creation failed");

    // Channels for dummy explorer
    let (tx_to_explorer, rx_explorer) = crossbeam_channel::bounded::<PlanetToExplorer>(1);

    // Use tx_orchestrator and rx_orchestrator for Planet <-> Orchestrator communications
    // Use tx_explorer and rx_explorer for Planet <-> Explorer communications

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the Planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator tells the Planet that an explorer arrived
    let _ = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::IncomingExplorerRequest {
            explorer_id: 0,
            new_sender: tx_to_explorer,
        },
    );

    // 4. Loop
    print!("Asteroids (A) and Sunrays (S) sequence : ");
    loop {
        iterations += 1;

        // a. Orchestrator sends a Sunray with probability SUNRAY_PROBABILITY - modifier, otherwise sends Asteroid
        if rand::random::<f32>() <= SUNRAY_PROBABILITY - sunray_probability_modifier {
            print!("S");
            let _ = orchestrator_send(
                &tx_orchestrator,
                &rx_orchestrator,
                OrchestratorToPlanet::Sunray(forge.generate_sunray()),
            );
            sunrays_received += 1;
        } else {
            print!("A");
            let response = orchestrator_send(
                &tx_orchestrator,
                &rx_orchestrator,
                OrchestratorToPlanet::Asteroid(forge.generate_asteroid()),
            );
            match response {
                PlanetToOrchestrator::AsteroidAck { rocket: None, .. } => break,

                PlanetToOrchestrator::AsteroidAck {
                    rocket: Some(_), ..
                } => asteroids_avoided += 1,

                _ => panic!("Expected AsteroidAck but received different response"),
            }
        }

        // b. Explorer asks Planet to create Hydrogen with probability EXPLORER_REQUEST_PROBABILITY, otherwise does nothing
        if rand::random::<f32>() <= EXPLORER_REQUEST_PROBABILITY {
            let msg = ExplorerToPlanet::GenerateResourceRequest {
                explorer_id: 0,
                resource: BasicResourceType::Hydrogen,
            };

            tx_explorer.send(msg).expect("Explorer failed to send");
            thread::sleep(Duration::from_millis(50));

            let response = rx_explorer.recv_timeout(Duration::from_millis(200));

            //let response = explorer_send(&tx_explorer, &rx_explorer, ExplorerToPlanet::GenerateResourceRequest {explorer_id : 0, resource : BasicResourceType::Hydrogen});

            // TODO: now it is allowed for the response to be an Err, since if ai decides not to fulfil the request it returns None
            // might need to modify this if that return is changed
            match response {
                Ok(PlanetToExplorer::GenerateResourceResponse { resource: None }) => {
                    refused_explorer_requests += 1
                }
                Ok(PlanetToExplorer::GenerateResourceResponse { resource: Some(_) }) => {
                    accepted_explorer_requests += 1
                }
                Err(_) => {
                    refused_explorer_requests += 1;
                    dont_have_energycell += 1;
                }
                _ => panic!("Expected GenerateResourceResponse but received different response"),
            }
        }
        // c. Increase sunray probability modifier to avoid endless loop
        sunray_probability_modifier += 0.001;
    }

    println!("\nPlanet is destroyed, concluding test.");
    println!("Iterations : {}", iterations);
    println!(
        "Accepted explorer's requests: {} - {}%",
        accepted_explorer_requests,
        ((accepted_explorer_requests as f32
            / (refused_explorer_requests as f32 + accepted_explorer_requests as f32))
            * 100.0) as u32
    );
    println!(
        "Refused explorer's requests: {} - {}%",
        refused_explorer_requests,
        ((refused_explorer_requests as f32
            / (refused_explorer_requests as f32 + accepted_explorer_requests as f32))
            * 100.0) as u32
    );
    println!(
        "Did not have EnergyCell: {} - {}% of refused",
        dont_have_energycell,
        ((dont_have_energycell as f32 / refused_explorer_requests as f32) * 100.0) as u32
    );
    println!("Asteroids avoided: {}", asteroids_avoided);
    println!(
        "Sunrays received: {} - {}% of Sunrays + Asteroids",
        sunrays_received,
        ((sunrays_received as f32 / (sunrays_received as f32 + asteroids_avoided as f32 + 1.0))
            * 100.0) as u32
    );
    println!(
        "Final Sunray probability: {}%",
        ((SUNRAY_PROBABILITY - sunray_probability_modifier) * 100.0) as u32
    );

    // 5. Orchestrator kills the Planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 6. Stop thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
