use std::time::Instant;

/// Tracks the frequency of sunray and asteroid events using Exponential Moving Average (EMA)
pub struct FrequencyCounter {
    /// Exponential Moving Average of sunray event frequency (events per second)
    sunray_rate: f32,
    /// Exponential Moving Average of asteroid event frequency (events per second)
    asteroid_rate: f32,
    /// Smoothing factor for sunray frequency EMA
    sunray_alpha: f32,
    /// Smoothing factor for asteroid frequency EMA
    asteroid_alpha: f32,
    /// Timestamp of the last sunray event
    last_sunray: Option<Instant>,
    /// Timestamp of the last asteroid event
    last_asteroid: Option<Instant>,
}

impl FrequencyCounter {
    /// Creates a new FrequencyCounter with all rates initialized to zero
    pub fn new() -> Self {
        FrequencyCounter {
            sunray_rate: 0.0,
            asteroid_rate: 0.5,
            sunray_alpha: 0.1,
            asteroid_alpha: 0.1,
            last_sunray: None,
            last_asteroid: None,
        }
    }

    /// Updates the Exponential Moving Average for sunray event frequency
    ///
    /// Uses an adaptive decay factor based on the current event rate to smooth the frequency estimate while remaining responsive to changes in event timing.
    pub fn update_sunray_rate(&mut self) {
        let now = Instant::now();

        if let Some(last) = self.last_sunray {
            let delta_time = now.duration_since(last).as_secs_f32();

            // EMA update: old decayed + new event contribution
            self.sunray_rate = self.sunray_rate * (1.0 - self.sunray_alpha)
                + self.sunray_alpha * (1.0 / delta_time);
        }

        // Update last event timestamp
        self.last_sunray = Some(now);
    }

    /// Updates the Exponential Moving Average for asteroid event frequency
    ///
    /// Uses an adaptive decay factor based on the current event rate to smooth the frequency estimate while remaining responsive to changes in event timing.
    pub fn update_asteroid_rate(&mut self) {
        let now = Instant::now();

        if let Some(last) = self.last_asteroid {
            let delta_time = now.duration_since(last).as_secs_f32();

            // EMA update: old decayed + new event contribution
            self.asteroid_rate = self.asteroid_rate * (1.0 - self.asteroid_alpha)
                + self.asteroid_alpha * (1.0 / delta_time);
        }

        // Update last event timestamp
        self.last_asteroid = Some(now);
    }
}
