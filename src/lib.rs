use common_game::components::planet::PlanetAI;
use common_game::components::planet::{self, PlanetState, PlanetType};
use common_game::components::resource::Generator;
use common_game::components::resource::{BasicResource, BasicResourceType, ComplexResourceType};
use common_game::components::resource::{
    Combinator, ComplexResource, ComplexResourceRequest, GenericResource,
};
use common_game::components::rocket::Rocket;
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use std::collections::HashSet;
use std::sync::mpsc;

//TODO: ADD RANDOM GENERATION LOGIC
#[allow(dead_code)]
struct Ai {
    is_ai_active: bool,
    random_mode: bool,
    rocket_gen_coeff: f32,
    basic_gen_coeff: f32,
    complex_gen_coeff: f32,
}

impl PlanetAI for Ai {
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        if self.is_ai_active {
            match msg {
                OrchestratorToPlanet::Sunray(sunray) => self.sunray_response(state, sunray),

                OrchestratorToPlanet::StartPlanetAI => self.start_planet_ai_response(state),

                OrchestratorToPlanet::StopPlanetAI => self.stop_planet_ai_response(state),

                OrchestratorToPlanet::InternalStateRequest => {
                    Some(PlanetToOrchestrator::InternalStateResponse {
                        planet_id: state.id(),
                        planet_state: state.to_dummy(),
                    })
                }

                _ => todo!(),
            }
        } else {
            None
        }
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        if self.is_ai_active {
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

                ExplorerToPlanet::CombineResourceRequest { msg, .. } => {
                    Some(PlanetToExplorer::CombineResourceResponse {
                        complex_response: self.combine_resource_response(state, combinator, msg),
                    })
                }

                ExplorerToPlanet::AvailableEnergyCellRequest { .. } => {
                    Some(PlanetToExplorer::AvailableEnergyCellResponse {
                        available_cells: { if state.cell(0).is_charged() { 1 } else { 0 } },
                    })
                }
            }
        } else {
            None
        }
    }

    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> Option<Rocket> {
        // If we have a rocket launch the rocket, otherwise if the energy cell is available use it to build the rocket and launch it, otherwise None
        if self.is_ai_active {
            if state.has_rocket() {
                state.take_rocket()
            } else if state.cell(0).is_charged() {
                let _ = state.build_rocket(0);
                state.take_rocket()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn start(&mut self, _state: &PlanetState) {
        self.is_ai_active = true;
    }

    fn stop(&mut self, _state: &PlanetState) {
        self.is_ai_active = false;
    }
}

impl Ai {
    pub fn new(
        random_mode: bool,
        rocket_gen_coeff: f32,
        basic_gen_coeff: f32,
        complex_gen_coeff: f32,
    ) -> Self {
        //check that coefficients are in bounds and eventually correct them
        let checked_basic_gen_coeff = basic_gen_coeff.clamp(0.0, 1.0);
        let checked_complex_gen_coeff = complex_gen_coeff.clamp(0.0, 1.0);
        let checked_rocket_gen_coeff = rocket_gen_coeff.clamp(0.0, 1.0);

        Ai {
            is_ai_active: false,
            random_mode,
            basic_gen_coeff: checked_basic_gen_coeff,
            complex_gen_coeff: checked_complex_gen_coeff,
            rocket_gen_coeff: checked_rocket_gen_coeff,
        }
    }

    ///Returns the available Basic Resources set of the planet
    fn supported_resource_response(&self, generator: &Generator) -> HashSet<BasicResourceType> {
        generator.all_available_recipes()
    }
    ///Returns the available Complex Resources set of the planet
    fn supported_combination_response(
        &self,
        combinator: &Combinator,
    ) -> HashSet<ComplexResourceType> {
        combinator.all_available_recipes()
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
        msg: ComplexResourceRequest,
    ) -> Result<ComplexResource, (String, GenericResource, GenericResource)> {
        if self.random_mode {
            todo!()
        }

        match msg {
            ComplexResourceRequest::Water(r1, r2) => combinator
                .make_water(r1, r2, state.cell_mut(0))
                .map(ComplexResource::Water)
                .map_err(|(s, r1, r2)| {
                    (
                        s,
                        GenericResource::BasicResources(BasicResource::Hydrogen(r1)),
                        GenericResource::BasicResources(BasicResource::Oxygen(r2)),
                    )
                }),

            ComplexResourceRequest::Diamond(r1, r2) => combinator
                .make_diamond(r1, r2, state.cell_mut(0))
                .map(ComplexResource::Diamond)
                .map_err(|(s, r1, r2)| {
                    (
                        s,
                        GenericResource::BasicResources(BasicResource::Carbon(r1)),
                        GenericResource::BasicResources(BasicResource::Carbon(r2)),
                    )
                }),

            ComplexResourceRequest::Life(r1, r2) => combinator
                .make_life(r1, r2, state.cell_mut(0))
                .map(ComplexResource::Life)
                .map_err(|(s, r1, r2)| {
                    (
                        s,
                        GenericResource::ComplexResources(ComplexResource::Water(r1)),
                        GenericResource::BasicResources(BasicResource::Carbon(r2)),
                    )
                }),
            ComplexResourceRequest::Robot(r1, r2) => combinator
                .make_robot(r1, r2, state.cell_mut(0))
                .map(ComplexResource::Robot)
                .map_err(|(s, r1, r2)| {
                    (
                        s,
                        GenericResource::BasicResources(BasicResource::Silicon(r1)),
                        GenericResource::ComplexResources(ComplexResource::Life(r2)),
                    )
                }),

            ComplexResourceRequest::Dolphin(r1, r2) => combinator
                .make_dolphin(r1, r2, state.cell_mut(0))
                .map(ComplexResource::Dolphin)
                .map_err(|(s, r1, r2)| {
                    (
                        s,
                        GenericResource::ComplexResources(ComplexResource::Water(r1)),
                        GenericResource::ComplexResources(ComplexResource::Life(r2)),
                    )
                }),

            ComplexResourceRequest::AIPartner(r1, r2) => combinator
                .make_aipartner(r1, r2, state.cell_mut(0))
                .map(ComplexResource::AIPartner)
                .map_err(|(s, r1, r2)| {
                    (
                        s,
                        GenericResource::ComplexResources(ComplexResource::Robot(r1)),
                        GenericResource::ComplexResources(ComplexResource::Diamond(r2)),
                    )
                }),
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
        })
    }

    fn start_planet_ai_response(
        &mut self,
        state: &mut PlanetState,
    ) -> Option<PlanetToOrchestrator> {
        self.start(state);
        Some(PlanetToOrchestrator::StartPlanetAIResult {
            planet_id: state.id(),
        })
    }

    fn stop_planet_ai_response(&mut self, state: &mut PlanetState) -> Option<PlanetToOrchestrator> {
        self.stop(state);
        Some(PlanetToOrchestrator::StopPlanetAIResult {
            planet_id: state.id(),
        })
    }
}

pub fn test() {
    let mut planet_ai = Ai::new(false, 0.0, 0.0, 0.0);
    planet_ai.is_ai_active = true;

    let (_orch_tx, orch_rx) = mpsc::channel::<OrchestratorToPlanet>();
    let (planet_to_orch_tx, _planet_to_orch_rx) = mpsc::channel::<PlanetToOrchestrator>();
    let (_explorer_tx, explorer_rx) = mpsc::channel::<ExplorerToPlanet>();
    let (_planet_to_explorer_tx, _planet_to_explorer_rx) = mpsc::channel::<PlanetToExplorer>();

    let planet = planet::Planet::new(
        0,
        PlanetType::C,
        Box::new(planet_ai),
        Vec::<BasicResourceType>::new(),
        Vec::<ComplexResourceType>::new(),
        (orch_rx, planet_to_orch_tx),
        explorer_rx,
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
        let mut planet_ai = Ai::new(false, 0.0, 0.0, 0.0);
        planet_ai.is_ai_active = true;

        let planet_ai_wrong_rocket_coeff = Ai::new(true, -0.7, 0.0, 0.0);
        let planet_ai_wrong_rocket_coeff_larger = Ai::new(true, 7.9, 0.0, 0.0);

        let planet_ai_wrong_basic_res_coeff = Ai::new(true, 0.7, -0.6, 0.0);
        let planet_ai_wrong_basic_res_coeff_larger = Ai::new(true, 0.7, 3.5, 0.0);

        let planet_ai_wrong_complex_res_coeff = Ai::new(true, 0.7, 0.6, -5.0);
        let planet_ai_wrong_complex_res_coeff_larger = Ai::new(true, 0.7, 0.6, 4.0);

        let (_orch_tx, orch_rx) = mpsc::channel::<OrchestratorToPlanet>();
        let (planet_to_orch_tx, _planet_to_orch_rx) = mpsc::channel::<PlanetToOrchestrator>();
        let (_explorer_tx, explorer_rx) = mpsc::channel::<ExplorerToPlanet>();
        let (_planet_to_explorer_tx, _planet_to_explorer_rx) = mpsc::channel::<PlanetToExplorer>();

        let planet = planet::Planet::new(
            0,
            PlanetType::C,
            Box::new(planet_ai),
            vec![BasicResourceType::Oxygen],
            vec![ComplexResourceType::Water],
            (orch_rx, planet_to_orch_tx),
            explorer_rx,
        );

        assert!(planet.is_ok(), "Planet creation should succeed");

        //Test correct creation of Ai with probability coefficients
        assert_eq!(
            planet_ai_wrong_rocket_coeff.rocket_gen_coeff, 0.0,
            "Rocket Coefficient should be 0"
        );
        assert_eq!(
            planet_ai_wrong_rocket_coeff_larger.rocket_gen_coeff, 1.0,
            "Rocket Coefficient should be 1"
        );

        assert_eq!(
            planet_ai_wrong_basic_res_coeff.basic_gen_coeff, 0.0,
            "Basic Resource Coefficient should be 0"
        );
        assert_eq!(
            planet_ai_wrong_basic_res_coeff_larger.basic_gen_coeff, 1.0,
            "Basic Resource Coefficient should be 1"
        );

        assert_eq!(
            planet_ai_wrong_complex_res_coeff.complex_gen_coeff, 0.0,
            "Complex Resource Coefficient should be 0"
        );
        assert_eq!(
            planet_ai_wrong_complex_res_coeff_larger.complex_gen_coeff, 1.0,
            "Complex Resource Coefficient should be 1"
        );
    }
}
