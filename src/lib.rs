use common_game::components::planet::PlanetAI;
use common_game::components::planet::{self, PlanetState, PlanetType};
use common_game::components::resource::{Combinator, ComplexResource, ComplexResourceRequest};
use common_game::components::resource::Generator;
use common_game::components::resource::{BasicResource, BasicResourceType, ComplexResourceType};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator};
use std::collections::HashSet;
use std::sync::mpsc;
use std::time::SystemTime;

struct Ai {}

impl planet::PlanetAI for Ai {
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        match msg {
            OrchestratorToPlanet::Sunray(sunray) => self.sunray_response(state, sunray),

            OrchestratorToPlanet::StartPlanetAI(_) => self.start_planet_ai_response(state),

            OrchestratorToPlanet::StopPlanetAI(_) => self.stop_planet_ai_response(state),

            OrchestratorToPlanet::InternalStateRequest(_) => {
                // TODO: InternalStateResponse requires owned PlanetState which we can't provide
                // We shoudl open an issue to discuss how to handle this on the common crate

                Some(PlanetToOrchestrator::InternalStateResponse {
                    planet_id: state.id(),
                    timestamp: SystemTime::now(),
                    planet_state: todo!(),
                })
            }

            _ => todo!(),
        }
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        match msg {
            ExplorerToPlanet::SupportedResourceRequest { .. } => {
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: self.supported_resource_response(generator),
                })
            }

            ExplorerToPlanet::SupportedCombinationRequest { .. } => {
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: self.supported_combination_response(combinator),
                })
            }

            ExplorerToPlanet::GenerateResourceRequest { resource, .. } => {
                Some(PlanetToExplorer::GenerateResourceResponse {
                    resource: self.generate_resource_response(state, generator, resource),
                })
            }

            ExplorerToPlanet::CombineResourceRequest {
                msg,
                ..
            } => {
                Some(PlanetToExplorer::CombineResourceResponse {
                    complex_response : self.combine_resource_response(state, combinator, msg)
                })
            }


            ExplorerToPlanet::AvailableEnergyCellRequest { .. } => {
                Some(PlanetToExplorer::AvailableEnergyCellResponse {
                    available_cells : {
                        if state.cell(0).is_charged() { 1 } else { 0 }
                    }
                })
            }

            ExplorerToPlanet::InternalStateRequest { .. } => {
                // TODO: Same as OrchestratorToPlanet::InternalStateRequest: InternalStateResponse requires owned PlanetState which we can't provide
                // We should open an issue to discuss how to handle this on the common crate

                Some(PlanetToExplorer::InternalStateResponse {
                    planet_state : todo!(),
                })
            }
        }
    }

    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket> {
        // If we have a rocket launch the rocket, otherwise if the energy cell is available use it to build the rocket and launch it, otherwise None
        if state.has_rocket() {
            state.take_rocket()
        } else if state.cell(0).is_charged() {
            let _ = state.build_rocket(0);
            state.take_rocket()
        } else {
            None
        }

    }

    fn start(&mut self, state: &PlanetState) {}

    fn stop(&mut self, state: &PlanetState) {}
}

impl Ai {
    ///Returns the available Basic Resources set of the planet
    fn supported_resource_response(
        &self,
        generator: &Generator,
    ) -> Option<HashSet<BasicResourceType>> {
        let available_resources = generator.all_available_recipes();
        if available_resources.is_empty() {
            None
        } else {
            Some(available_resources)
        }
    }
    ///Returns the available Complex Resources set of the planet
    fn supported_combination_response(
        &self,
        combinator: &Combinator,
    ) -> Option<HashSet<ComplexResourceType>> {
        let available_comp_resources = combinator.all_available_recipes();
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
        generator: &Generator,
        to_generate: BasicResourceType,
    ) -> Option<BasicResource> {
        match to_generate {
            BasicResourceType::Carbon => generator
                .make_carbon(state.cell_mut(0))
                .ok()
                .map(BasicResource::Carbon),

            BasicResourceType::Hydrogen => generator
                .make_hydrogen(state.cell_mut(0))
                .ok()
                .map(BasicResource::Hydrogen),

            BasicResourceType::Silicon => generator
                .make_silicon(state.cell_mut(0))
                .ok()
                .map(BasicResource::Silicon),

            BasicResourceType::Oxygen => generator
                .make_oxygen(state.cell_mut(0))
                .ok()
                .map(BasicResource::Oxygen),
        }
    }

    // Returns the optional complex resource created
    fn combine_resource_response(
        &self,
        state: &mut PlanetState,
        combinator: &Combinator,
        msg : ComplexResourceRequest
    ) -> Option<ComplexResource> {
        match msg {
            ComplexResourceRequest::Water(r1,r2) => combinator
                .make_water(r1, r2, state.cell_mut(0))
                .ok()
                .map(ComplexResource::Water),

            ComplexResourceRequest::Diamond(r1, r2) => combinator
                .make_diamond(r1, r2, state.cell_mut(0))
                .ok()
                .map(ComplexResource::Diamond),

            ComplexResourceRequest::Life(r1, r2) => combinator
                .make_life(r1, r2, state.cell_mut(0))
                .ok()
                .map(ComplexResource::Life),

            ComplexResourceRequest::Robot(r1, r2) => combinator
                .make_robot(r1, r2, state.cell_mut(0))
                .ok()
                .map(ComplexResource::Robot),

            ComplexResourceRequest::Dolphin(r1, r2) => combinator
                .make_dolphin(r1, r2, state.cell_mut(0))
                .ok()
                .map(ComplexResource::Dolphin),

            ComplexResourceRequest::AIPartner(r1, r2) => combinator
                .make_aipartner(r1, r2, state.cell_mut(0))
                .ok()
                .map(ComplexResource::AIPartner)
        }
    }

    fn sunray_response(
        &self,
        state: &mut PlanetState,
        sunray: Sunray,
    ) -> Option<PlanetToOrchestrator> {
        if state.cell(0).is_charged() && state.has_rocket() {
            let _ = state.build_rocket(0); // Currently the planet doesn't have an Option<Rocket> but can only generate one when needed
        }

        state.cell_mut(0).charge(sunray);

        Some(PlanetToOrchestrator::SunrayAck {
            planet_id: state.id(),
            timestamp: SystemTime::now(),
        })
    }

    fn start_planet_ai_response(
        &mut self,
        state: &mut PlanetState,
    ) -> Option<PlanetToOrchestrator> {
        self.start(state);
        Some(PlanetToOrchestrator::StartPlanetAIResult {
            planet_id: state.id(),
            timestamp: SystemTime::now(),
        })
    }

    fn stop_planet_ai_response(&mut self, state: &mut PlanetState) -> Option<PlanetToOrchestrator> {
        self.stop(state);
        Some(PlanetToOrchestrator::StopPlanetAIResult {
            planet_id: state.id(),
            timestamp: SystemTime::now(),
        })
    }
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