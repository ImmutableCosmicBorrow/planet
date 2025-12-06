use std::time::Instant;

/// Tracks the relative likelihood of sunray and asteroid events
pub struct FrequencyCounter {
    /// Count of sunray events
    sunray_count: u32,
    /// Count of asteroid events
    asteroid_count: u32,
    /// Timestamp of the last sunray event
    last_sunray: Option<Instant>,
    /// Timestamp of the last asteroid event
    last_asteroid: Option<Instant>,
    /// Exponent for importance weighting (kept from original for compatibility)
    sunray_importance: f32,
}

impl FrequencyCounter {
    /// Creates a new FrequencyCounter with initial counts for Bayesian smoothing
    /// Starts with 100% asteroid
    pub fn new() -> Self {
        FrequencyCounter {
            sunray_count: 0,
            asteroid_count: 1, // 100% asteroid prior
            last_sunray: None,
            last_asteroid: None,
            sunray_importance: 0.5,
        }
    }

    /// Call this when a sunray event occurs
    pub fn update_sunray(&mut self) {
        self.last_sunray = Some(Instant::now());
        self.sunray_count += 1;
    }

    /// Call this when an asteroid event occurs
    pub fn update_asteroid(&mut self) {
        self.last_asteroid = Some(Instant::now());
        self.asteroid_count += 1;
    }

    /// Returns the probability that the next event is a sunray
    pub fn p_sunray(&self) -> f32 {
        let alpha_s = self.sunray_count as f32;
        let alpha_a = self.asteroid_count as f32;

        if alpha_s + alpha_a == 0.0 {
            0.5 // fallback
        } else {
            // Optionally apply importance exponent if you want to keep the original behavior
            alpha_s.powf(self.sunray_importance)
                / (alpha_s.powf(self.sunray_importance)
                    + alpha_a.powf(1.0 - self.sunray_importance))
        }
    }

    /// Returns the probability that the next event is an asteroid
    pub fn p_asteroid(&self) -> f32 {
        1.0 - self.p_sunray()
    }
}
