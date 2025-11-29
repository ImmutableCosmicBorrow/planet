use common_game::components::planet::{self, PlanetState, PlanetType};
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::components::rocket::Rocket;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use std::sync::mpsc;

struct Ai {}

impl planet::PlanetAI for Ai {
    fn handle_orchestrator_msg(
        &mut self,
        _state: &mut PlanetState,
        _msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        None
    }

    fn handle_explorer_msg(
        &mut self,
        _state: &mut PlanetState,
        _msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        None
    }

    fn handle_asteroid(&mut self, _state: &mut PlanetState) -> Option<Rocket> {
        None
    }

    fn start(&mut self, _state: &PlanetState) {}

    fn stop(&mut self) {}
}

pub fn test() {
    let planet_ai = Ai {};
    let (_orch_tx, orch_rx) = mpsc::channel::<OrchestratorToPlanet>();
    let (planet_to_orch_tx, _planet_to_orch_rx) = mpsc::channel::<PlanetToOrchestrator>();
    let (_explorer_tx, explorer_rx) = mpsc::channel::<ExplorerToPlanet>();
    let (planet_to_explorer_tx, _planet_to_explorer_rx) = mpsc::channel::<PlanetToExplorer>();

    let planet = planet::Planet::new(
        0,
        PlanetType::C,
        planet_ai,
        Vec::<BasicResourceType>::new(),
        Vec::<ComplexResourceType>::new(),
        (orch_rx, planet_to_orch_tx),
        (explorer_rx, planet_to_explorer_tx),
    );

    match planet {
        Ok(_) => println!("Planet created successfully"),
        Err(e) => println!("Error creating planet: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_creation() {
        let planet_ai = Ai {};
        let (_orch_tx, orch_rx) = mpsc::channel::<OrchestratorToPlanet>();
        let (planet_to_orch_tx, _planet_to_orch_rx) = mpsc::channel::<PlanetToOrchestrator>();
        let (_explorer_tx, explorer_rx) = mpsc::channel::<ExplorerToPlanet>();
        let (planet_to_explorer_tx, _planet_to_explorer_rx) = mpsc::channel::<PlanetToExplorer>();

        let planet = planet::Planet::new(
            0,
            PlanetType::C,
            planet_ai,
            vec![BasicResourceType::Oxygen],
            vec![ComplexResourceType::Water],
            (orch_rx, planet_to_orch_tx),
            (explorer_rx, planet_to_explorer_tx),
        );

        assert!(planet.is_ok(), "Planet creation should succeed");
    }
}
