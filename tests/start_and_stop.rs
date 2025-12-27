mod common;

use common::*;
use common_game::components::forge::Forge;
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};

#[test]
fn start_and_stop() {
    let (planet, (tx_orchestrator, rx_orchestrator), _tx_explorer) = create_test_planet();

    let forge = Forge::new().unwrap();

    // 1. Create thread
    let _handle = start_thread(planet);

    // 2. Orchestrator starts planet
    orchestrator_start_planet(&tx_orchestrator, &rx_orchestrator);

    // 3. Orchestrator sends 2 sunrays (so planet should build a rocket)
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Sunray(forge.generate_sunray()),
    );
    assert!(
        matches!(response, PlanetToOrchestrator::SunrayAck { .. }),
        "Expected SunrayAck but got a different message"
    );

    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::StopPlanetAI,
    );
    assert!(
        matches!(response, PlanetToOrchestrator::StopPlanetAIResult { .. }),
        "Expected StopPlanetAIResult but got a different message"
    );

    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::StartPlanetAI,
    );
    assert!(
        matches!(response, PlanetToOrchestrator::StartPlanetAIResult { .. }),
        "Expected StartPlanetAIResult but got a different message"
    );
}
