use super::Ai;
use common_game::components::planet::{PlanetAI, PlanetState};
use common_game::components::resource::{Combinator, Generator};
use common_game::components::sunray::Sunray;
use common_game::protocols::messages::{OrchestratorToPlanet, PlanetToOrchestrator};

pub(super) fn handle_message(
    ai: &mut Ai,
    state: &mut PlanetState,
    _generator: &Generator,
    _combinator: &Combinator,
    msg: OrchestratorToPlanet,
) -> Option<PlanetToOrchestrator> {
    match msg {
        OrchestratorToPlanet::Sunray(sunray) => handle_sunray(ai, state, sunray),

        OrchestratorToPlanet::StartPlanetAI => handle_start_ai(ai, state),

        OrchestratorToPlanet::StopPlanetAI => handle_stop_ai(ai, state),

        OrchestratorToPlanet::InternalStateRequest => handle_internal_state_request(ai, state),

        _ => {
            // Other messages are currently handled by the planet
            None
        }
    }
}

fn handle_sunray(
    ai: &mut Ai,
    state: &mut PlanetState,
    sunray: Sunray,
) -> Option<PlanetToOrchestrator> {
    if state.cell(0).is_charged() && !state.has_rocket() {
        let _ = state.build_rocket(0);
    }

    state.cell_mut(0).charge(sunray);

    if let Some(counters) = ai.counters_mut() {
        counters.update_sunray();
    }

    Some(PlanetToOrchestrator::SunrayAck {
        planet_id: state.id(),
    })
}

fn handle_start_ai(ai: &mut Ai, state: &mut PlanetState) -> Option<PlanetToOrchestrator> {
    ai.start(state);
    Some(PlanetToOrchestrator::StartPlanetAIResult {
        planet_id: state.id(),
    })
}

fn handle_stop_ai(ai: &mut Ai, state: &mut PlanetState) -> Option<PlanetToOrchestrator> {
    ai.stop(state);
    Some(PlanetToOrchestrator::StopPlanetAIResult {
        planet_id: state.id(),
    })
}

fn handle_internal_state_request(
    _ai: &mut Ai,
    state: &mut PlanetState,
) -> Option<PlanetToOrchestrator> {
    Some(PlanetToOrchestrator::InternalStateResponse {
        planet_id: state.id(),
        planet_state: state.to_dummy(),
    })
}
