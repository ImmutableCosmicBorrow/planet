#![allow(clippy::pedantic)]

use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::ExplorerToPlanet;
use crossbeam_channel::unbounded;
use immutable_cosmic_borrow::create_planet;
use std::time::Duration;
/// Test that a planet can be successfully created using the create_planet function
#[test]
fn test_planet_creation() {
    let (_tx_orch_in, rx_orch_in) = unbounded::<OrchestratorToPlanet>();
    // Channel 2: Planet -> Orchestrator
    let (tx_orch_out, _rx_orch_out) = unbounded::<PlanetToOrchestrator>();

    // Channel 3: Explorer -> Planet
    let (_tx_expl_in, rx_expl_in) = unbounded::<ExplorerToPlanet>();

    // Create a planet with basic resource (Oxygen) and complex resource (Water) generation capabilities
    // Fill in the missing arguments with placeholder/example values as per the function signature
    // Example: (bool, f32, f32, Duration, Duration, i32, (Receiver, Sender), Receiver)
    let planet = create_planet(
        false,                      // Example bool
        0.0,                        // Example f32
        0.0,                        // Example f32
        Duration::from_secs(1),     // Example Duration
        Duration::from_millis(100), // Example Duration
        2,                          // Example i32 (number of resources)
        (rx_orch_in, tx_orch_out),  // Channels
        rx_expl_in,                 // Channel
    );

    assert!(
        planet.is_ok(),
        "Planet creation should succeed, but got: {:?}",
        planet.err()
    );
}
