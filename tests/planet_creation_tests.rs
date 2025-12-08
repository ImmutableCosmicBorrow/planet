use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToOrchestrator,
};
use crossbeam_channel::unbounded;
use planet::{Ai, create_planet};
/// Test that a planet can be successfully created using the create_planet function
#[test]
fn test_planet_creation() {
    // Create an AI with all coefficients set to 0 (no random generation)
    let planet_ai = Ai::new(false, 0.0, 0.0);

    let (_tx_orch_in, rx_orch_in) = unbounded::<OrchestratorToPlanet>();
    // Channel 2: Planet -> Orchestrator
    let (tx_orch_out, _rx_orch_out) = unbounded::<PlanetToOrchestrator>();

    // Channel 3: Explorer -> Planet
    let (_tx_expl_in, rx_expl_in) = unbounded::<ExplorerToPlanet>();

    // Create a planet with basic resource (Oxygen) and complex resource (Water) generation capabilities
    let planet = create_planet(
        planet_ai,
        vec![BasicResourceType::Oxygen],
        vec![ComplexResourceType::Water],
        (rx_orch_in, tx_orch_out),
        rx_expl_in,
    );

    assert!(
        planet.is_ok(),
        "Planet creation should succeed, but got: {:?}",
        planet.err()
    );
}
