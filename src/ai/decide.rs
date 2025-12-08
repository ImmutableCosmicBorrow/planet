use super::Ai;

pub fn generate_basic_resource(ai: &mut Ai) -> bool {
    if ai.random_mode() {
        rand::random::<f32>() > ai.basic_gen_coeff
    } else if let Some(counters) = &mut ai.counters {
        counters.p_sunray() > (1 - ai.basic_gen_coeff)
    } else {
        false
    }
}

pub fn generate_complex_resource(ai: &mut Ai) -> bool {
    if ai.random_mode() {
        rand::random::<f32>() > ai.complex_gen_coeff
    } else if let Some(counters) = &mut ai.counters {
        counters.p_sunray() > (1 - ai.complex_gen_coeff)
    } else {
        false
    }
}
