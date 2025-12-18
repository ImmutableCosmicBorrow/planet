#![allow(clippy::pedantic)]

use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::ExplorerToPlanet;
use crossbeam_channel::unbounded;
use planet::{Ai, create_planet};
use std::time::Duration;
/// Test that a planet can be successfully created using the create_planet function
#[test]
fn test_planet_creation() {
    // Create an AI with all coefficients set to 0 (no random generation)
    let planet_ai = Ai::new(
        false,
        0.0,
        0.0,
        Duration::from_secs(1),
        Duration::from_millis(100),
    );

    let (_tx_orch_in, rx_orch_in) = unbounded::<OrchestratorToPlanet>();
    // Channel 2: Planet -> Orchestrator
    let (tx_orch_out, _rx_orch_out) = unbounded::<PlanetToOrchestrator>();

    // Channel 3: Explorer -> Planet
    let (_tx_expl_in, rx_expl_in) = unbounded::<ExplorerToPlanet>();

    // Create a planet with basic resource (Oxygen) and complex resource (Water) generation capabilities
    let planet = create_planet(planet_ai, (rx_orch_in, tx_orch_out), rx_expl_in);

    assert!(
        planet.is_ok(),
        "Planet creation should succeed, but got: {:?}",
        planet.err()
    );
}
