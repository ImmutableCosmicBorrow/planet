use std::time::Instant;

/// Tracks the adaptive probability of sunray vs asteroid events
pub struct FrequencyCounter {
    /// Exponentially weighted probability of the next event being sunray
    p_sunray: f32,
    /// Timestamp of the last event (any type)
    last_update: Option<Instant>,
    /// Decay timescale (in seconds) controlling how fast older events are forgotten
    tau: f32,
}

impl FrequencyCounter {
    /// Creates a new FrequencyCounter with initial probabilities
    /// `tau` is the decay timescale in seconds
    pub fn new(tau: f32) -> Self {
        FrequencyCounter {
            p_sunray: 0.01, // start with low likelihood of sunray
            last_update: None,
            tau,
        }
    }

    /// Updates the counter when a sunray event occurs
    pub fn update_sunray(&mut self) {
        self.update_event(true);
    }

    /// Updates the counter when an asteroid event occurs
    pub fn update_asteroid(&mut self) {
        self.update_event(false);
    }

    /// Internal update function: `is_sunray` = true for sunray, false for asteroid
    fn update_event(&mut self, is_sunray: bool) {
        self.update_no_event();

        // Incorporate the new event
        let event_value = if is_sunray { 1.0 } else { 0.0 };
        self.p_sunray += (1.0 - self.p_sunray) * event_value;
        // clamp to [0,1] just in case
        self.p_sunray = self.p_sunray.clamp(0.0, 1.0);
    }

    /// Internal update function for no event (used for decay over time)
    fn update_no_event(&mut self) {
        let now = Instant::now();
        // Compute time since last update in seconds
        let delta_t = if let Some(last) = self.last_update {
            now.duration_since(last).as_secs_f32()
        } else {
            0.0
        };
        self.last_update = Some(now);
        // Exponential decay based on elapsed time
        let decay_factor = (-delta_t / self.tau).exp();
        self.p_sunray *= decay_factor;
    }

    /// Returns the probability that the next event is sunray
    pub fn p_sunray(&mut self) -> f32 {
        self.update_no_event();
        self.p_sunray
    }
}
