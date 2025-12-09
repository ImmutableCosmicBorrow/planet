use planet::Ai;
use std::time::Duration;

/// Test that AI coefficients within the valid range [0.0, 1.0] are preserved
#[test]
fn planet_ai_valid_coefficient_creation() {
    // Test coefficients at boundaries
    let planet_ai_min = Ai::new(true, 0.0, 0.0, Duration::from_secs(1));
    let planet_ai_max = Ai::new(false, 1.0, 1.0, Duration::from_secs(1));

    // Test coefficients in the middle of the range
    let planet_ai_mid = Ai::new(true, 0.5, 0.7, Duration::from_secs(1));
    // Verify that valid coefficients are preserved exactly
    assert_eq!(
        planet_ai_min.basic_gen_coeff(),
        0.0,
        "Basic resource coefficient at minimum should be 0.0"
    );
    assert_eq!(
        planet_ai_min.complex_gen_coeff(),
        0.0,
        "Complex resource coefficient at minimum should be 0.0"
    );

    assert_eq!(
        planet_ai_max.basic_gen_coeff(),
        1.0,
        "Basic resource coefficient at maximum should be 1.0"
    );
    assert_eq!(
        planet_ai_max.complex_gen_coeff(),
        1.0,
        "Complex resource coefficient at maximum should be 1.0"
    );

    assert_eq!(
        planet_ai_mid.basic_gen_coeff(),
        0.5,
        0.5,
        "Basic resource coefficient in range should be preserved"
    );
    assert_eq!(
        planet_ai_mid.complex_gen_coeff(),
        0.7,
        0.7,
        "Complex resource coefficient in range should be preserved"
    );
}

/// Test that AI coefficients are correctly clamped to the valid range [0.0, 1.0]
#[test]
fn planet_ai_wrong_coefficient_creation() {
    // Test coefficients outside valid range (should be clamped)
    let test_cases = [
        ((-0.7, 0.0), (0.0, 0.0)),
        ((7.9, 0.0), (1.0, 0.0)),
        ((0.7, -0.6), (0.7, 0.0)),
        ((0.7, 3.5), (0.7, 1.0)),
        ((0.7, 0.6), (0.7, 0.6)),
        ((0.7, 0.6), (0.7, 0.6)),
    ];

    for ((basic_in, complex_in), (basic_out, complex_out)) in test_cases {
        let ai = Ai::new(true, basic_in, complex_in, Duration::from_secs(1));

        assert_eq!(
            ai.basic_gen_coeff(),
            basic_out,
            "Basic resource coefficient {} should be clamped to {}",
            basic_in,
            basic_out
        );
        assert_eq!(
            ai.complex_gen_coeff(),
            complex_out,
            "Complex resource coefficient {} should be clamped to {}",
            complex_in,
            complex_out
        );
    }
}
