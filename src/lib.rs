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

use common_game::components;
use common_game::components::planet::PlanetState;
use common_game::components::resource::{BasicResource, BasicResourceType, ComplexResourceType};
use common_game::components::rocket::Rocket;
use common_game::protocols;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use std::collections::HashSet;
use std::mem;

pub struct PlanetAi;
impl components::planet::PlanetAI for PlanetAi {
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        todo!()
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        match msg {
            ExplorerToPlanet::SupportedResourceRequest { explorer_id } => {
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: self.supported_resource_response(state),
                })
            }

            ExplorerToPlanet::SupportedCombinationRequest { explorer_id } => {
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: self.supported_combination_response(state),
                })
            }

            ExplorerToPlanet::GenerateResourceRequest {
                explorer_id,
                resource,
            } => Some(PlanetToExplorer::GenerateResourceResponse {
                resource: self.generate_resource_response(state, resource),
            }),

            _ => todo!(),
        }
    }

    fn handle_asteroid(&mut self, state: &mut PlanetState) -> Option<Rocket> {
        todo!()
    }

    fn start(&mut self, state: &PlanetState) {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }
}

impl PlanetAi {
    ///Returns the available Basic Resources set of the planet
    fn supported_resource_response(
        &self,
        state: &PlanetState,
    ) -> Option<HashSet<BasicResourceType>> {
        let available_resources = state.generator.all_available_recipes();
        if available_resources.is_empty() {
            None
        } else {
            Some(available_resources)
        }
    }
    ///Returns the available Complex Resources set of the planet
    fn supported_combination_response(
        &self,
        state: &PlanetState,
    ) -> Option<HashSet<ComplexResourceType>> {
        let available_comp_resources = state.combinator.all_available_recipes();
        if available_comp_resources.is_empty() {
            None
        } else {
            Some(available_comp_resources)
        }
    }

    ///Return the optional Basic resource generated
    fn generate_resource_response(
        &self,
        state: &mut PlanetState,
        to_generate: BasicResourceType,
    ) -> Option<BasicResource> {

        let generator = mem::take(&mut state.generator);

        match to_generate {

            BasicResourceType::Carbon => {
                let res = generator
                    .make_carbon(state.cell_mut(0))
                    .ok()
                    .map(BasicResource::Carbon);
                state.generator = generator;
                res

            }

            BasicResourceType::Hydrogen => {
                let res = generator
                    .make_hydrogen(state.cell_mut(0))
                    .ok()
                    .map(BasicResource::Hydrogen);
                state.generator = generator;
                res
            }

            BasicResourceType::Silicon => {
                let res = generator
                    .make_silicon(state.cell_mut(0))
                    .ok()
                    .map(BasicResource::Silicon);
                state.generator = generator;
                res
            }

            BasicResourceType::Oxygen => {
                let res = generator.make_oxygen(state.cell_mut(0)).ok().map(BasicResource::Oxygen);
                state.generator = generator;
                res
            }


        }

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
