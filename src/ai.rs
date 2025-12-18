mod asteroid;
mod decide;
mod explorer;
mod orchestrator;

use crate::frequency_counter::FrequencyCounter;
use common_game::components::planet::PlanetAI;
use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use common_game::components::rocket::Rocket;
use common_game::protocols::orchestrator_planet::PlanetToOrchestrator;
use common_game::protocols::planet_explorer::{ExplorerToPlanet, PlanetToExplorer};
use std::time::Duration;

pub struct Ai {
    is_ai_active: bool,
    random_mode: bool,
    pub(crate) basic_gen_coeff: f32,
    pub(crate) complex_gen_coeff: f32,
    counters: Option<FrequencyCounter>,
}

impl PlanetAI for Ai {
    fn handle_explorer_msg(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
        msg: ExplorerToPlanet,
    ) -> Option<PlanetToExplorer> {
        // Delegate to explorer::handle_message
        explorer::handle_message(self, state, generator, combinator, msg)
    }

    fn handle_asteroid(
        &mut self,
        state: &mut PlanetState,
        generator: &Generator,
        combinator: &Combinator,
    ) -> Option<Rocket> {
        // Delegate to asteroid::handle_asteroid
        asteroid::handle_asteroid(self, state, generator, combinator)
    }

    fn handle_sunray(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        sunray: common_game::components::sunray::Sunray,
    ) {
        // Delegate to orchestrator::handle_sunray
        orchestrator::handle_sunray(self, state, sunray);
    }

    fn handle_internal_state_req(
        &mut self,
        state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
    ) -> common_game::components::planet::DummyPlanetState {
        if let PlanetToOrchestrator::InternalStateResponse { planet_state, .. } =
            orchestrator::handle_internal_state_request(self, state)
        {
            return planet_state;
        }
        state.to_dummy()
    }

    fn on_explorer_arrival(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        _explorer_id: common_game::utils::ID,
    ) {
    }

    fn on_explorer_departure(
        &mut self,
        _state: &mut PlanetState,
        _generator: &Generator,
        _combinator: &Combinator,
        _explorer_id: common_game::utils::ID,
    ) {
    }

    fn on_start(&mut self, state: &PlanetState, _generator: &Generator, _combinator: &Combinator) {
        orchestrator::handle_start_ai(self, state);
    }

    fn on_stop(&mut self, state: &PlanetState, _generator: &Generator, _combinator: &Combinator) {
        orchestrator::handle_stop_ai(self, state);
    }
}

impl Ai {
    #[must_use]
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
    #[must_use]
    pub fn basic_gen_coeff(&self) -> f32 {
        self.basic_gen_coeff
    }

    #[must_use]
    pub fn complex_gen_coeff(&self) -> f32 {
        self.complex_gen_coeff
    }
}
