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
pub struct Ai {
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
            let _ = state.build_rocket(0);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that AI coefficients within the valid range [0.0, 1.0] are preserved
    #[test]
    fn planet_ai_valid_coefficient_creation() {
        // Test coefficients at boundaries
        let planet_ai_min = Ai::new(true, 0.0, 0.0, 0.0);
        let planet_ai_max = Ai::new(false, 1.0, 1.0, 1.0);

        // Test coefficients in the middle of the range
        let planet_ai_mid = Ai::new(true, 0.5, 0.7, 0.3);

        // Verify that valid coefficients are preserved exactly
        assert_eq!(
            planet_ai_min.rocket_gen_coeff, 0.0,
            "Rocket coefficient at minimum should be 0.0"
        );
        assert_eq!(
            planet_ai_min.basic_gen_coeff, 0.0,
            "Basic resource coefficient at minimum should be 0.0"
        );
        assert_eq!(
            planet_ai_min.complex_gen_coeff, 0.0,
            "Complex resource coefficient at minimum should be 0.0"
        );

        assert_eq!(
            planet_ai_max.rocket_gen_coeff, 1.0,
            "Rocket coefficient at maximum should be 1.0"
        );
        assert_eq!(
            planet_ai_max.basic_gen_coeff, 1.0,
            "Basic resource coefficient at maximum should be 1.0"
        );
        assert_eq!(
            planet_ai_max.complex_gen_coeff, 1.0,
            "Complex resource coefficient at maximum should be 1.0"
        );

        assert_eq!(
            planet_ai_mid.rocket_gen_coeff, 0.5,
            "Rocket coefficient in range should be preserved"
        );
        assert_eq!(
            planet_ai_mid.basic_gen_coeff, 0.7,
            "Basic resource coefficient in range should be preserved"
        );
        assert_eq!(
            planet_ai_mid.complex_gen_coeff, 0.3,
            "Complex resource coefficient in range should be preserved"
        );
    }

    /// Test that AI coefficients are correctly clamped to the valid range [0.0, 1.0]
    #[test]
    fn planet_ai_wrong_coefficient_creation() {
        // Test coefficients outside valid range (should be clamped)
        let test_cases = [
            ((-0.7, 0.0, 0.0), (0.0, 0.0, 0.0)),
            ((7.9, 0.0, 0.0), (1.0, 0.0, 0.0)),
            ((0.7, -0.6, 0.0), (0.7, 0.0, 0.0)),
            ((0.7, 3.5, 0.0), (0.7, 1.0, 0.0)),
            ((0.7, 0.6, -5.0), (0.7, 0.6, 0.0)),
            ((0.7, 0.6, 4.0), (0.7, 0.6, 1.0)),
        ];

        for ((rocket_in, basic_in, complex_in), (rocket_out, basic_out, complex_out)) in test_cases
        {
            let ai = Ai::new(true, rocket_in, basic_in, complex_in);

            assert_eq!(
                ai.rocket_gen_coeff, rocket_out,
                "Rocket coefficient {} should be clamped to {}",
                rocket_in, rocket_out
            );
            assert_eq!(
                ai.basic_gen_coeff, basic_out,
                "Basic resource coefficient {} should be clamped to {}",
                basic_in, basic_out
            );
            assert_eq!(
                ai.complex_gen_coeff, complex_out,
                "Complex resource coefficient {} should be clamped to {}",
                complex_in, complex_out
            );
        }
    }

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
}
