use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToOrchestrator,
};
use planet::{Ai, create_planet};
use std::sync::mpsc;

/// Test that a planet can be successfully created using the create_planet function
#[test]
fn test_planet_creation() {
    // Create an AI with all coefficients set to 0 (no random generation)
    let planet_ai = Ai::new(false, 0.0, 0.0, 0.0);

    // Set up communication channels for orchestrator
    let (_orch_tx, orch_rx) = mpsc::channel::<OrchestratorToPlanet>();
    let (planet_to_orch_tx, _planet_to_orch_rx) = mpsc::channel::<PlanetToOrchestrator>();

    // Set up communication channel for explorers
    let (_explorer_tx, explorer_rx) = mpsc::channel::<ExplorerToPlanet>();

    // Create a planet with basic resource (Oxygen) and complex resource (Water) generation capabilities
    let planet = create_planet(
        planet_ai,
        vec![BasicResourceType::Oxygen],
        vec![ComplexResourceType::Water],
        (orch_rx, planet_to_orch_tx),
        explorer_rx,
    );

    assert!(
        planet.is_ok(),
        "Planet creation should succeed, but got: {:?}",
        planet.err()
    );
}
