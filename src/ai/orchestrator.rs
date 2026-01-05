use crate::ai;

use super::Ai;
use common_game::components::planet::PlanetState;
use common_game::components::sunray::Sunray;
use common_game::logging::{Channel, EventType, Payload};
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

    let counter_payload = if let Some(counters) = ai.counters_mut() {
        counters.update_sunray();

        let (sun_intensity, asteroid_intensity) = counters.debug_stats();
        let probability = counters.sunray_probability();

        let mut payload = Payload::new();
        payload.insert("action".into(), "update_sunray_counter".into());
        payload.insert("sun_intensity".into(), format!("{sun_intensity:.6}"));
        payload.insert(
            "asteroid_intensity".into(),
            format!("{asteroid_intensity:.6}"),
        );
        payload.insert("sunray_probability".into(), format!("{probability:.6}"));

        Some(payload)
    } else {
        None
    };

    if let Some(payload) = counter_payload {
        ai::Ai::log_planet_event(
            state,
            Some(ai::Ai::orchestrator_participant()),
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
    }

    let mut ack_payload = Payload::new();
    ack_payload.insert("message".into(), "SunrayAck".into());
    ai::Ai::log_planet_event(
        state,
        Some(ai::Ai::orchestrator_participant()),
        EventType::MessagePlanetToOrchestrator,
        Channel::Trace,
        ack_payload,
    );

    PlanetToOrchestrator::SunrayAck {
        planet_id: state.id(),
    }
}

pub(crate) fn handle_start_ai(ai: &mut Ai, state: &PlanetState) -> PlanetToOrchestrator {
    ai.is_ai_active = true;
    if let Some(counter) = ai.counters_mut() {
        counter.restart();
    }

    let mut ack_payload = Payload::new();
    ack_payload.insert("message".into(), "StartPlanetAIResult".into());
    ai::Ai::log_planet_event(
        state,
        Some(ai::Ai::orchestrator_participant()),
        EventType::MessagePlanetToOrchestrator,
        Channel::Trace,
        ack_payload,
    );
    PlanetToOrchestrator::StartPlanetAIResult {
        planet_id: state.id(),
    }
}

pub(crate) fn handle_stop_ai(ai: &mut Ai, state: &PlanetState) -> PlanetToOrchestrator {
    ai.is_ai_active = false;
    if let Some(counter) = ai.counters_mut() {
        counter.stop();
    }

    let mut ack_payload = Payload::new();
    ack_payload.insert("message".into(), "StopPlanetAIResult".into());
    ai::Ai::log_planet_event(
        state,
        Some(ai::Ai::orchestrator_participant()),
        EventType::MessagePlanetToOrchestrator,
        Channel::Trace,
        ack_payload,
    );
    PlanetToOrchestrator::StopPlanetAIResult {
        planet_id: state.id(),
    }
}

pub(crate) fn handle_internal_state_request(state: &mut PlanetState) -> PlanetToOrchestrator {
    let mut response_payload = Payload::new();
    response_payload.insert("message".into(), "InternalStateResponse".into());
    ai::Ai::log_planet_event(
        state,
        Some(ai::Ai::orchestrator_participant()),
        EventType::MessagePlanetToOrchestrator,
        Channel::Debug,
        response_payload,
    );
    PlanetToOrchestrator::InternalStateResponse {
        planet_id: state.id(),
        planet_state: state.to_dummy(),
    }
}
