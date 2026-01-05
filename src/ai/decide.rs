use crate::ai;

use super::Ai;
use common_game::components::planet::PlanetState;
use common_game::logging::{Channel, EventType, Payload};

pub fn generate_basic_resource(ai: &mut Ai, state: &PlanetState) -> bool {
    // Check if energy cell is charged first
    if !state.cell(0).is_charged() {
        let mut payload = Payload::new();
        payload.insert("action".into(), "generate_basic_resource".into());
        payload.insert("reason".into(), "cell_not_charged".into());
        ai::Ai::log_planet_event(
            state,
            None,
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
        return false;
    }

    let mut payload = Payload::new();
    payload.insert("action".into(), "generate_basic_resource".into());
    payload.insert("random_mode".into(), ai.random_mode().to_string());
    payload.insert("has_rocket".into(), state.has_rocket().to_string());

    let decision = if ai.random_mode() {
        let sample = rand::random::<f32>();
        payload.insert("random_sample".into(), format!("{sample:.6}"));
        payload.insert("threshold".into(), format!("{:.6}", ai.basic_gen_coeff));
        sample > ai.basic_gen_coeff
    } else if let Some(counters) = &mut ai.counters {
        // Use sunray probability to decide: help if asteroid risk is low enough
        let p_sunray = counters.sunray_probability();
        let p_asteroid = 1.0 - p_sunray;

        payload.insert("p_sunray".into(), format!("{p_sunray:.6}"));
        payload.insert("p_asteroid".into(), format!("{p_asteroid:.6}"));

        // If we have a rocket, evaluate risk of 2 asteroids before next sunray
        if state.has_rocket() {
            let p_squared = p_asteroid * p_asteroid;
            payload.insert("p_asteroid_squared".into(), format!("{p_squared:.6}"));
            p_squared <= ai.basic_gen_coeff
        } else {
            // No rocket - evaluate single asteroid risk
            p_asteroid <= ai.basic_gen_coeff
        }
    } else {
        false
    };

    payload.insert("decision".into(), decision.to_string());
    ai::Ai::log_planet_event(
        state,
        None,
        EventType::InternalPlanetAction,
        Channel::Debug,
        payload,
    );

    decision
}

pub fn generate_complex_resource(ai: &mut Ai, state: &PlanetState) -> bool {
    // Check if energy cell is charged first
    if !state.cell(0).is_charged() {
        let mut payload = Payload::new();
        payload.insert("action".into(), "generate_complex_resource".into());
        payload.insert("reason".into(), "cell_not_charged".into());
        ai::Ai::log_planet_event(
            state,
            None,
            EventType::InternalPlanetAction,
            Channel::Debug,
            payload,
        );
        return false;
    }

    let mut payload = Payload::new();
    payload.insert("action".into(), "generate_complex_resource".into());
    payload.insert("random_mode".into(), ai.random_mode().to_string());
    payload.insert("has_rocket".into(), state.has_rocket().to_string());

    let decision = if ai.random_mode() {
        let sample = rand::random::<f32>();
        payload.insert("random_sample".into(), format!("{sample:.6}"));
        payload.insert("threshold".into(), format!("{:.6}", ai.complex_gen_coeff));
        sample > ai.complex_gen_coeff
    } else if let Some(counters) = &mut ai.counters {
        // Use sunray probability to decide: help if asteroid risk is low enough
        let p_sunray = counters.sunray_probability();
        let p_asteroid = 1.0 - p_sunray;

        payload.insert("p_sunray".into(), format!("{p_sunray:.6}"));
        payload.insert("p_asteroid".into(), format!("{p_asteroid:.6}"));

        // If we have a rocket, evaluate risk of 2 asteroids before next sunray
        if state.has_rocket() {
            let p_squared = p_asteroid * p_asteroid;
            payload.insert("p_asteroid_squared".into(), format!("{p_squared:.6}"));
            p_squared <= ai.complex_gen_coeff
        } else {
            // No rocket - evaluate single asteroid risk
            p_asteroid <= ai.complex_gen_coeff
        }
    } else {
        false
    };

    payload.insert("decision".into(), decision.to_string());
    ai::Ai::log_planet_event(
        state,
        None,
        EventType::InternalPlanetAction,
        Channel::Debug,
        payload,
    );

    decision
}
