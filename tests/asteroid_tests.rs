mod utils;

use std::thread;
use common_game::components::asteroid::Asteroid;
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{OrchestratorToPlanet, PlanetToOrchestrator};
use utils::*;

#[test]
fn test_handle_asteroid(){
    let (mut planet,
        (tx_orchestrator, rx_orchestrator),
        _tx_explorer) = create_test_planet();

    // Create thread
    let handle = thread::spawn(move || {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = planet.run();
        }));
    });

    // Orchestrator starts planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator sends 2 sunrays (so planet should build a rocket)
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));
    assert!(match response {
        PlanetToOrchestrator::SunrayAck { .. } => true,
        PlanetToOrchestrator::AsteroidAck { .. } => panic!("Got AsteroidAck"),
        PlanetToOrchestrator::InternalStateResponse { .. } => panic!("Got InternalStateResponse"),
        PlanetToOrchestrator::StartPlanetAIResult { .. } => panic!("Got StartPlanetAIResult"),
        PlanetToOrchestrator::StopPlanetAIResult { .. } => panic!("Got StopPlanetAIResult"),
        PlanetToOrchestrator::KillPlanetResult { .. } => panic!("Got KillPlanetResult"),

        _ => false,
    }, "Expected SunrayAck but got a different message");

    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));
    assert!(match response {
        PlanetToOrchestrator::SunrayAck { .. } => true,
        _ => false
    }, "Expected SunrayAck but got a different message");


    // Orchestrator sends an asteroid
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Asteroid(Asteroid::default()));

    // Planet should respond with Rocket
    match response{
        PlanetToOrchestrator::AsteroidAck {rocket, .. } => assert!(rocket.is_some(), "Expected Rocket but got None, planet would be destroyed"),
        _ => panic!("Expected AsteroidAck but got a different message"),
    }

    // Orchestrator sends an asteroid
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Asteroid(Asteroid::default()));

    // Planet should respond with Rocket
    match response{
        PlanetToOrchestrator::AsteroidAck {rocket, ..} => assert!(rocket.is_some(), "Expected Rocket but got None, planet would be destroyed"),
        _ => panic!("Expected AsteroidAck but got a different message"),
    }

    // Orchestrator sends an asteroid
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Asteroid(Asteroid::default()));

    // Planet should respond with None
    match response{
        PlanetToOrchestrator::AsteroidAck {rocket, ..} => assert!(rocket.is_none(), "Expected None but got Rocket, but planet did not have a rocket"),
        _ => panic!("Expected AsteroidAck but got a different message"),
    }

    drop(tx_orchestrator);
    let _ = handle.join();
}
