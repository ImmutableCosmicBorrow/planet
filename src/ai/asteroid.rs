use crate::ai::Ai;
use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use common_game::components::rocket::Rocket;

pub fn handle_asteroid(
    ai: &mut Ai,
    state: &mut PlanetState,
    _generator: &Generator,
    _combinator: &Combinator,
) -> Option<Rocket> {
    if let Some(counters) = ai.counters_mut() {
        counters.update_asteroid(std::time::Instant::now());
    }
    if state.has_rocket() {
        state.take_rocket()
    } else if state.cell(0).is_charged() {
        let _ = state.build_rocket(0);
        state.take_rocket()
    } else {
        None
    }
}
