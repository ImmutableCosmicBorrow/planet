use super::Ai;
use common_game::components::planet::PlanetState;
use common_game::components::sunray::Sunray;
use common_game::protocols::orchestrator_planet::PlanetToOrchestrator;

pub(crate) fn handle_sunray(
    ai: &mut Ai,
    state: &mut PlanetState,
    sunray: Sunray,
) -> PlanetToOrchestrator {
    if state.cell(0).is_charged() && !state.has_rocket() {
        let _ = state.build_rocket(0);
    }

    state.cell_mut(0).charge(sunray);

    if let Some(counters) = ai.counters_mut() {
        counters.update_sunray();
    }

    PlanetToOrchestrator::SunrayAck {
        planet_id: state.id(),
    }
}

pub(crate) fn handle_start_ai(ai: &mut Ai, state: &PlanetState) -> PlanetToOrchestrator {
    ai.is_ai_active = true;
    if let Some(counter) = ai.counters_mut() {
        counter.restart();
    }
    PlanetToOrchestrator::StartPlanetAIResult {
        planet_id: state.id(),
    }
}

pub(crate) fn handle_stop_ai(ai: &mut Ai, state: &PlanetState) -> PlanetToOrchestrator {
    ai.is_ai_active = false;
    if let Some(counter) = ai.counters_mut() {
        counter.stop();
    }
    PlanetToOrchestrator::StopPlanetAIResult {
        planet_id: state.id(),
    }
}

pub(crate) fn handle_internal_state_request(
    _ai: &mut Ai,
    state: &mut PlanetState,
) -> PlanetToOrchestrator {
    PlanetToOrchestrator::InternalStateResponse {
        planet_id: state.id(),
        planet_state: state.to_dummy(),
    }
}
