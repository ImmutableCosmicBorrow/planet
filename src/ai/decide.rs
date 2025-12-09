use super::Ai;
use common_game::components::planet::PlanetState;

pub fn generate_basic_resource(ai: &mut Ai, state: &PlanetState) -> bool {
    // Check if energy cell is charged first
    if !state.cell(0).is_charged() {
        return false;
    }

    if ai.random_mode() {
        rand::random::<f32>() > ai.basic_gen_coeff
    } else if let Some(counters) = &mut ai.counters {
        // Use sunray probability to decide: help if asteroid risk is low enough
        let p_sunray = counters.sunray_probability();
        let p_asteroid = 1.0 - p_sunray;
        
        // If we have a rocket, evaluate risk of 2 asteroids before next sunray
        if state.has_rocket() {
            let p_squared = p_asteroid * p_asteroid;
            p_squared <= ai.basic_gen_coeff
        } else {
            // No rocket - evaluate single asteroid risk
            p_asteroid <= ai.basic_gen_coeff
        }
    } else {
        false
    }
}

pub fn generate_complex_resource(ai: &mut Ai, state: &PlanetState) -> bool {
    // Check if energy cell is charged first
    if !state.cell(0).is_charged() {
        return false;
    }

    if ai.random_mode() {
        rand::random::<f32>() > ai.complex_gen_coeff
    } else if let Some(counters) = &mut ai.counters {
        // Use sunray probability to decide: help if asteroid risk is low enough
        let p_sunray = counters.sunray_probability();
        let p_asteroid = 1.0 - p_sunray;
        
        // If we have a rocket, evaluate risk of 2 asteroids before next sunray
        if state.has_rocket() {
            let p_squared = p_asteroid * p_asteroid;
            p_squared <= ai.complex_gen_coeff
        } else {
            // No rocket - evaluate single asteroid risk
            p_asteroid <= ai.complex_gen_coeff
        }
    } else {
        false
    }
}
