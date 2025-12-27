use common_game::components::planet::{self, PlanetType};
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::protocols::orchestrator_planet::{OrchestratorToPlanet, PlanetToOrchestrator};
use common_game::protocols::planet_explorer::ExplorerToPlanet;
use common_game::utils::ID;
use crossbeam_channel::{Receiver, Sender};

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
/// # Errors
/// * `Err(String)` - Error message if planet creation fails (e.g., empty `gen_rules`)
pub fn create_planet(
    planet_ai: Ai,
    id: ID,
    orchestrator_channels: (Receiver<OrchestratorToPlanet>, Sender<PlanetToOrchestrator>),
    explorers_receiver: Receiver<ExplorerToPlanet>,
) -> Result<planet::Planet, String> {
    planet::Planet::new(
        id,
        PlanetType::C,
        Box::new(planet_ai),
        vec![BasicResourceType::Hydrogen],
        vec![
            ComplexResourceType::AIPartner,
            ComplexResourceType::Diamond,
            ComplexResourceType::Dolphin,
            ComplexResourceType::Life,
            ComplexResourceType::Robot,
            ComplexResourceType::Water,
        ],
        orchestrator_channels,
        explorers_receiver,
    )
}
