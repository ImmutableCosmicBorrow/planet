mod utils;

use std::thread;
use std::time::Duration;
use common_game::components::asteroid::Asteroid;
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{OrchestratorToPlanet, PlanetToOrchestrator};
use crate::utils::*;

#[test]
fn test_sunray_response(){
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        _) = create_test_planet();

    // start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator sends a sunray
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));

    // Planet should respond with a SunrayAck
    match response {
        PlanetToOrchestrator::SunrayAck { planet_id: _ } => { },
        _ => panic!{"Expected a SunrayAck response but received a different one"},
    }

    // Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_asteroid_response(){
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        _) = create_test_planet();

    // start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // Orchestrator sends a sunray
    let _ = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Sunray(Sunray::default()));


    // Orchestrator sends an asteroid
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::Asteroid(Asteroid::default()));

    // Planet should respond with a rocket
    match response{
        PlanetToOrchestrator::AsteroidAck {rocket, .. } => assert!(rocket.is_some(), "Expected a rocket but Planet responded with None"),
        _ => panic!{"Expected a AsteroidAck but received a different one"},
    }

    // Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_start_stop_response(){
    let (planet,
        (tx_orchestrator, rx_orchestrator),
        _tx_explorer) = create_test_planet();

    // Start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::StartPlanetAI);

    // Planet should respond with a StartPlanetAIResult
    match response{
        PlanetToOrchestrator::StartPlanetAIResult {planet_id: _ } => { },
        _ => panic!("Expected a StartPlanetAIResult but received a different one")
    }
    //
    thread::sleep(Duration::from_millis(200));

    // Orchestrator stops the planet
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::StopPlanetAI);

    // Planet should respond with a StopPlanetAIResult
    match response{
        PlanetToOrchestrator::StopPlanetAIResult {planet_id: _ } => { },
        _ => panic!("Expected a StopPlanetAIResult but received a different one")
    }

    drop(tx_orchestrator);
    let _ = handle.join();
}
#[test]
fn test_internal_state_response(){

    let (planet,
        (tx_orchestrator, rx_orchestrator),
        _) = create_test_planet();

    // start thread
    let handle = start_thread(planet);

    // Orchestrator starts the planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // TODO: now the test fails because InternalStateRequest is not implemented yet
    // Orchestrator asks for internal state
    let response = orchestrator_send(&tx_orchestrator, &rx_orchestrator, OrchestratorToPlanet::InternalStateRequest);

    // Planet should respond with its internal state


    match response {
        PlanetToOrchestrator::InternalStateResponse {..} => { },
        _ => panic!{"Expected a InternalStateResponse but received a different one"},
    }


    // Orchestrator stops planet
    orchestrator_stop_planet(&tx_orchestrator, &rx_orchestrator);


    drop(tx_orchestrator);
    let _ = handle.join();
}
