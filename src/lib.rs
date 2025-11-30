use common_game::components::planet::{self, PlanetState, PlanetType};
use common_game::components::resource::{BasicResource, BasicResourceType, ComplexResourceType};
use common_game::components::rocket::Rocket;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use std::collections::HashSet;
use std::mem;
use std::sync::mpsc;

struct Ai {}

impl planet::PlanetAI for Ai {
    fn handle_orchestrator_msg(
        &mut self,
        _state: &mut PlanetState,
        _msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        match msg {
            OrchestratorToPlanet::Sunray(sunray) => {
                self.sunray_response(state, sunray)
            }

            OrchestratorToPlanet::Asteroid(asteroid) => {
                // Handle Asteroid message
                let rocket = self.handle_asteroid(state);
                Some(PlanetToOrchestrator::AsteroidAck {
                    planet_id: state.id,
                    rocket,
                })
            }

            OrchestratorToPlanet::StartPlanetAI(_) => {
                self.start(state);
                Some(PlanetToOrchestrator::StartPlanetAIResult {
                    planet_id: state.id,
                    timestamp: SystemTime::now(),
                })
            }

            OrchestratorToPlanet::StopPlanetAI(_) => {
                self.stop();
                Some(PlanetToOrchestrator::StopPlanetAIResult {
                    planet_id: state.id,
                    timestamp: SystemTime::now(),
                })
            }

            OrchestratorToPlanet::InternalStateRequest(_) => {
                Some(PlanetToOrchestrator::InternalStateResponse {
                    planet_id: state.id,
                    planet_state: state.clone(),
                    timestamp: SystemTime::now(),
                })
            }

            _ => todo!(),
        }
    }

    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        match msg {
            ExplorerToPlanet::SupportedResourceRequest { .. } => {
                Some(PlanetToExplorer::SupportedResourceResponse {
                    resource_list: self.supported_resource_response(state),
                })
            }

            ExplorerToPlanet::SupportedCombinationRequest { .. } => {
                Some(PlanetToExplorer::SupportedCombinationResponse {
                    combination_list: self.supported_combination_response(state),
                })
            }

            ExplorerToPlanet::GenerateResourceRequest { resource, .. } => {
                Some(PlanetToExplorer::GenerateResourceResponse {
                    resource: self.generate_resource_response(state, resource),
                })
            }

            _ => todo!(),
        }
    }

    fn handle_asteroid(&mut self, _state: &mut PlanetState) -> Option<Rocket> {
        None
    }

    fn start(&mut self, _state: &PlanetState) {}

    fn stop(&mut self) {}
}

impl Ai {
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
        let generator = mem::take(&mut state.generator); //TODO: Understand if this is the right way of using Generator

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
                let res = generator
                    .make_oxygen(state.cell_mut(0))
                    .ok()
                    .map(BasicResource::Oxygen);
                state.generator = generator;
                res
            }
        }
    }
}

pub enum OrchestratorToPlanet {
    Sunray(Sunray),
    Asteroid(Asteroid),
    StartPlanetAI(StartPlanetAiMsg),
    StopPlanetAI(StopPlanetAiMsg),
    InternalStateRequest(InternalStateRequestMsg), //I think orchestrator should always have the internal state for the UI, but up to discussions
}

impl Ai {
    fn sunray_response(&self, _state: &mut PlanetState, _sunray: Sunray) -> Option<PlanetToOrchestrator> {
        _state.cell_mut(0).charge(_sunray);

        if()
        
        Some(PlanetToOrchestrator::SunrayAck {
            planet_id: _state.id,
            timestamp: SystemTime::now(),
        })
    }
}

pub enum PlanetToOrchestrator {
    SunrayAck {
        planet_id: u32,
        timestamp: SystemTime,
    },
    AsteroidAck {
        planet_id: u32,
        rocket: Option<Rocket>,
    }, //depends on how we want to manage the defense + TODO add timestamp but planet code complains
    StartPlanetAIResult {
        planet_id: u32,
        timestamp: SystemTime,
    },
    StopPlanetAIResult {
        planet_id: u32,
        timestamp: SystemTime,
    },
    InternalStateResponse {
        planet_id: u32,
        planet_state: PlanetState,
        timestamp: SystemTime,
    }, //do we want to clone the planetState?, orchestrator should always know the planetState
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
    use common_game::components::resource::BasicResource::Oxygen;

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

    #[test]
    fn test_supported_resource_response() {
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
        )
        .unwrap();

        let expected_res = HashSet::from([BasicResourceType::Oxygen]);
        let av_resource = planet.ai.supported_resource_response(&planet.state());

        assert!(av_resource.is_some(), "Expected Some resources");
        assert_eq!(
            av_resource.unwrap(),
            expected_res,
            "Resources should match expected resources"
        );
    }

    #[test]
    fn test_supported_combination_response() {
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
        )
        .unwrap();

        let expected_res = HashSet::from([ComplexResourceType::Water]);
        let av_complex = planet
            .ai
            .supported_combination_response(&planet.state())
            .unwrap();
        assert!(
            planet
                .ai
                .supported_combination_response(&planet.state())
                .is_some(),
            "Expected Some complex resources"
        );
        assert_eq!(
            av_complex, expected_res,
            "Expected resources should match expected complex resources"
        );
    }

    //rewrite test once I know how to use resource generation in the right way
    /*#[test]
    fn generate_resource_response() {
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
        ).unwrap();

        let planet_state = planet.state().clone();

        let expected_res = Some(BasicResource::Oxygen);
        let generated_resource = planet.ai.generate_resource_response(&mut planet_state, BasicResourceType::Oxygen);
    }*/
}
