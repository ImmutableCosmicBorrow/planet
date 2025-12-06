use std::time::Instant;

/// Tracks the frequency of sunray and asteroid events using Exponential Moving Average (EMA)
pub struct FrequencyCounter {
    /// Exponential Moving Average of sunray event frequency (events per second)
    sunray_rate: f32,
    /// Exponential Moving Average of asteroid event frequency (events per second)
    asteroid_rate: f32,
    /// Half-life for sunray EMA decay (in seconds)
    sunray_half_life: f32,
    /// Half-life for asteroid EMA decay (in seconds)
    asteroid_half_life: f32,
    /// Timestamp of the last sunray event
    last_sunray: Option<Instant>,
    /// Timestamp of the last asteroid event
    last_asteroid: Option<Instant>,
    /// Exponent for importance weighting between sunray and asteroid frequencies
    sunray_importance: f32,
}

impl FrequencyCounter {
    /// Creates a new FrequencyCounter with rates initialized to default values.
    pub fn new() -> Self {
        FrequencyCounter {
            sunray_rate: 0.0,
            asteroid_rate: 0.5,
            sunray_half_life: 0.5,
            asteroid_half_life: 1.0,
            last_sunray: None,
            last_asteroid: None,
            sunray_importance: 0.5,
        }
    }

    /// Updates the sunray event EMA rate based on the current time elapsed since the last event.
    pub fn update_sunray_rate(&mut self) {
        let now = Instant::now();

        if let Some(last) = self.last_sunray {
            let delta_t = now.duration_since(last).as_secs_f32();
            if delta_t > 0.0 {
                // compute alpha based on time difference
                let alpha = 1.0 - (-delta_t / self.sunray_half_life).exp();
                // update EMA
                self.sunray_rate = alpha * (1.0 / delta_t) + (1.0 - alpha) * self.sunray_rate;
            }
        }
        self.last_sunray = Some(now);
    }

    /// Updates the asteroid event EMA rate based on the current time elapsed since the last event.
    pub fn update_asteroid_rate(&mut self) {
        let now = Instant::now();

        if let Some(last) = self.last_asteroid {
            let delta_t = now.duration_since(last).as_secs_f32();
            if delta_t > 0.0 {
                // compute alpha based on time difference
                let alpha = 1.0 - (-delta_t / self.asteroid_half_life).exp();
                // update EMA
                self.asteroid_rate = alpha * (1.0 / delta_t) + (1.0 - alpha) * self.asteroid_rate;
            }
        }
        self.last_asteroid = Some(now);
    }

    pub fn compare_frequencies(&self) -> f32 {
        if self.asteroid_rate == 0.0 && self.sunray_rate == 0.0 {
            0.5
        } else {
            self.sunray_rate.powf(self.sunray_importance) / (self.sunray_rate.powf(self.sunray_importance) + self.asteroid_rate.powf(1.0 - self.sunray_importance))
        }
    }
}

