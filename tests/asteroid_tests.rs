mod common;

use common::*;
use common_game::components::forge::Forge;
use common_game::protocols::messages::{OrchestratorToPlanet, PlanetToOrchestrator};

#[test]
fn test_handle_asteroid() {
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
        OrchestratorToPlanet::Sunray(forge.generate_sunray()),
    );
    assert!(
        match response {
            PlanetToOrchestrator::SunrayAck { .. } => true,
            _ => false,
        },
        "Expected SunrayAck but got a different message"
    );

    // 4. Orchestrator sends an asteroid
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Asteroid(forge.generate_asteroid()),
    );

    // 5. Planet should respond with Rocket
    match response {
        PlanetToOrchestrator::AsteroidAck { rocket, .. } => assert!(
            rocket.is_some(),
            "Expected Rocket but got None, planet would be destroyed"
        ),
        _ => panic!("Expected AsteroidAck but got a different message"),
    }

    // 6. Orchestrator sends an asteroid
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Asteroid(forge.generate_asteroid()),
    );

    // 7. Planet should respond with Rocket
    match response {
        PlanetToOrchestrator::AsteroidAck { rocket, .. } => assert!(
            rocket.is_some(),
            "Expected Rocket but got None, planet would be destroyed"
        ),
        _ => panic!("Expected AsteroidAck but got a different message"),
    }

    // 8. Orchestrator sends an asteroid
    let response = orchestrator_send(
        &tx_orchestrator,
        &rx_orchestrator,
        OrchestratorToPlanet::Asteroid(forge.generate_asteroid()),
    );

    // 9. Planet should respond with None
    match response {
        PlanetToOrchestrator::AsteroidAck { rocket, .. } => assert!(
            rocket.is_none(),
            "Expected None but got Rocket, but planet did not have a rocket"
        ),
        _ => panic!("Expected AsteroidAck but got a different message"),
    }

    // 10. Orchestrator kills planet
    orchestrator_kill_planet(&tx_orchestrator, &rx_orchestrator);

    drop(tx_orchestrator);
    let _ = handle.join();
}
