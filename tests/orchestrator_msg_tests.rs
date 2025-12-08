mod common;

use crate::common::*;
use common_game::components::asteroid::Asteroid;
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{OrchestratorToPlanet, PlanetToOrchestrator};
use std::thread;
use std::time::Duration;

#[test]
fn test_sunray_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), _) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator sends a sunray
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    // 4. Planet should respond with a SunrayAck
    match response {
        PlanetToOrchestrator::SunrayAck { planet_id: _ } => {}
        _ => panic! {"Expected a SunrayAck response but received a different one"},
    }

    // 5. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 6. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_asteroid_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), _) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator sends a sunray
    let _ = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(Sunray::default()),
    );

    // 4. Orchestrator sends an asteroid
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Asteroid(Asteroid::default()),
    );

    // 5. Planet should respond with a rocket
    match response {
        PlanetToOrchestrator::AsteroidAck { rocket, .. } => assert!(
            rocket.is_some(),
            "Expected a rocket but Planet responded with None"
        ),
        _ => panic! {"Expected a AsteroidAck but received a different one"},
    }

    // 6. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 7. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_start_stop_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), _tx_explorer) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::StartPlanetAI,
    );

    // 3. Planet should respond with a StartPlanetAIResult
    match response {
        PlanetToOrchestrator::StartPlanetAIResult { planet_id: _ } => {}
        _ => panic!("Expected a StartPlanetAIResult but received a different one"),
    }
    //
    thread::sleep(Duration::from_millis(200));

    // 4. Orchestrator stops the planet
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::StopPlanetAI,
    );

    // 5. Planet should respond with a StopPlanetAIResult
    match response {
        PlanetToOrchestrator::StopPlanetAIResult { planet_id: _ } => {}
        _ => panic!("Expected a StopPlanetAIResult but received a different one"),
    }

    // 6. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 7. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_internal_state_response() {
    let (planet, (tx_orchestrator, rx_orchestrator), _) = create_test_planet();

    // 1. Start thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator asks for internal state
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::InternalStateRequest,
    );

    // 4. Planet should respond with its internal state
    match response {
        PlanetToOrchestrator::InternalStateResponse { .. } => {}
        _ => panic! {"Expected a InternalStateResponse but received a different one"},
    }

    // 5. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    // 6. End thread
    drop(tx_orchestrator);
    let _ = handle.join();
}
