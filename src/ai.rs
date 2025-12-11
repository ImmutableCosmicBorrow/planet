mod asteroid;
mod decide;
mod explorer;
mod orchestrator;

use crate::frequency_counter::FrequencyCounter;
use common_game::components::planet::PlanetAI;
use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use common_game::components::rocket::Rocket;
use common_game::protocols::messages::{
    ExplorerToPlanet, OrchestratorToPlanet, PlanetToExplorer, PlanetToOrchestrator,
};
use std::time::Duration;

pub struct Ai {
    is_ai_active: bool,
    random_mode: bool,
    pub(crate) basic_gen_coeff: f32,
    pub(crate) complex_gen_coeff: f32,
    counters: Option<FrequencyCounter>,
}

impl PlanetAI for Ai {
    fn handle_orchestrator_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: OrchestratorToPlanet,
    ) -> Option<PlanetToOrchestrator> {
        if self.is_ai_active {
            orchestrator::handle_message(self, state, generator, combinator, msg)
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
            explorer::handle_message(self, state, generator, combinator, msg)
        } else {
            None
        }
    }

    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket> {
        if self.is_ai_active {
            asteroid::handle_asteroid(self, state, generator, combinator)
        } else {
            None
        }
    }

    fn start(&mut self, _state: &PlanetState) {
        self.is_ai_active = true;
        self.counters.as_mut().unwrap().restart();
    }

    fn stop(&mut self, _state: &PlanetState) {
        self.is_ai_active = false;
        self.counters.as_mut().unwrap().stop();
    }
}

impl Ai {
    pub fn new(
        random_mode: bool,
        basic_gen_coeff: f32,
        complex_gen_coeff: f32,
        half_life: Duration,
        min_time_constant: Duration,
    ) -> Self {
        //check that coefficients are in bounds and eventually correct them
        let checked_basic_gen_coeff = basic_gen_coeff.clamp(0.0, 1.0);
        let checked_complex_gen_coeff = complex_gen_coeff.clamp(0.0, 1.0);

        Ai {
            is_ai_active: false,
            random_mode,
            basic_gen_coeff: checked_basic_gen_coeff,
            complex_gen_coeff: checked_complex_gen_coeff,
            counters: Some(FrequencyCounter::new(half_life, min_time_constant)),
        }
    }

    pub(crate) fn counters_mut(&mut self) -> &mut Option<FrequencyCounter> {
        &mut self.counters
    }

    pub(crate) fn random_mode(&self) -> bool {
        self.random_mode
    }

    // Public getters for testing
    pub fn basic_gen_coeff(&self) -> f32 {
        self.basic_gen_coeff
    }

    pub fn complex_gen_coeff(&self) -> f32 {
        self.complex_gen_coeff
    }
}
