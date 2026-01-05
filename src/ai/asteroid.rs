use crate::ai::{self, Ai};
use common_game::components::planet::PlanetState;
use common_game::components::resource::{Combinator, Generator};
use common_game::components::rocket::Rocket;
use common_game::logging::{Channel, EventType, Payload};

pub fn handle_asteroid(
    ai: &mut Ai,
    state: &mut PlanetState,
    _generator: &Generator,
    _combinator: &Combinator,
) -> Option<Rocket> {
    if !ai.is_ai_active {
        let mut payload = Payload::new();
        payload.insert("action".into(), "ignore_asteroid_ai_inactive".into());
        ai::Ai::log_planet_event(
            state,
            Some(ai::Ai::orchestrator_participant()),
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
        return None;
    }
    let counter_payload = if let Some(counters) = ai.counters_mut() {
        counters.update_asteroid();

        let (sun_intensity, asteroid_intensity) = counters.debug_stats();
        let probability = counters.sunray_probability();

        let mut payload = Payload::new();
        payload.insert("action".into(), "update_asteroid_counter".into());
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
    if state.has_rocket() {
        let mut payload = Payload::new();
        payload.insert("action".into(), "launch_existing_rocket".into());
        ai::Ai::log_planet_event(
            state,
            Some(ai::Ai::orchestrator_participant()),
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
        state.take_rocket()
    } else if state.cell(0).is_charged() {
        let _ = state.build_rocket(0);
        let mut payload = Payload::new();
        payload.insert("action".into(), "build_and_launch_rocket".into());
        ai::Ai::log_planet_event(
            state,
            Some(ai::Ai::orchestrator_participant()),
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
        state.take_rocket()
    } else {
        let mut payload = Payload::new();
        payload.insert("action".into(), "no_rocket_available".into());
        ai::Ai::log_planet_event(
            state,
            Some(ai::Ai::orchestrator_participant()),
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
        None
    }
}
