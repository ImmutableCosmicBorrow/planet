use common_game::components::planet::{self, PlanetType};
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToOrchestrator,
};
use std::sync::mpsc;

mod ai;
mod frequency_counter;

pub use ai::Ai;

/// Creates a new Planet instance with the provided AI and communication channels.
///
/// # Arguments
/// * `planet_ai` - The AI implementation that will control the planet's behavior
/// * `gen_rules` - Vector of basic resource types that the planet can generate (must not be empty)
/// * `comb_rules` - Vector of complex resource types that the planet can combine
/// * `orchestrator_channels` - Tuple of (receiver, sender) for communication with the orchestrator
/// * `explorers_receiver` - Receiver channel for messages from explorers
///
/// # Returns
/// * `Ok(Planet)` - Successfully created planet with ID 0 and type C
/// * `Err(String)` - Error message if planet creation fails (e.g., empty gen_rules)
pub fn create_planet(
    planet_ai: Ai,
    gen_rules: Vec<BasicResourceType>,
    comb_rules: Vec<ComplexResourceType>,
    orchestrator_channels: (
        mpsc::Receiver<OrchestratorToPlanet>,
        mpsc::Sender<PlanetToOrchestrator>,
    ),
    explorers_receiver: mpsc::Receiver<ExplorerToPlanet>,
) -> Result<planet::Planet, String> {
    planet::Planet::new(
        0,
        PlanetType::C,
        Box::new(planet_ai),
        gen_rules,
        comb_rules,
        orchestrator_channels,
        explorers_receiver,
    )
}
