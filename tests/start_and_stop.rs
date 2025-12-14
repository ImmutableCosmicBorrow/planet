mod common;

use common::*;
use common_game::components::forge::Forge;
use common_game::protocols::messages::{OrchestratorToPlanet, PlanetToOrchestrator};

#[test]
fn start_and_stop() {
    let (planet, (tx_orchestrator, rx_orchestrator), _tx_explorer) = create_test_planet();

    let forge = Forge::new().unwrap();

    // 1. Create thread
    let handle = start_thread(planet);

    // 2. Orchestrator starts planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator sends 2 sunrays (so planet should build a rocket)
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(forge.generate_sunray()),
    );
    assert!(
        match response {
            PlanetToOrchestrator::SunrayAck { .. } => true,
            _ => false,
        },
        "Expected SunrayAck but got a different message"
    );

    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::StopPlanetAI,
    );
    assert!(
        match response {
            PlanetToOrchestrator::StopPlanetAIResult { .. } => true,
            _ => false,
        },
        "Expected StopPlanetAIResult but got a different message"
    );

    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::StartPlanetAI,
    );
    assert!(
        match response {
            PlanetToOrchestrator::StartPlanetAIResult { .. } => true,
            _ => false,
        },
        "Expected StartPlanetAIResult but got a different message"
    );
}
