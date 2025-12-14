use std::time::{Duration, Instant};

pub struct FrequencyCounter {
    // Half-life for exponential decay
    half_life: Duration,
    impulse: f32,

    // Competing intensities
    sun_intensity: f32,
    asteroid_intensity: f32,

    // Timing
    last_update: Option<Instant>,

    // Probability of sunray
    sunray_probability: f32,

    // Minimum time constant
    min_time_constant: Duration,

    // Stop time tracking
    stop_time: Option<Instant>,
    restart_time: Option<Instant>,
}

impl FrequencyCounter {
    pub fn new(half_life: Duration, min_time_constant: Duration) -> Self {
        let tau = half_life.as_secs_f32() / std::f32::consts::LN_2;
        let impulse = 1.0 / tau;

        Self {
            half_life,
            impulse,
            sun_intensity: 0.5,
            asteroid_intensity: 0.5,
            last_update: None,
            sunray_probability: 0.5,
            min_time_constant,
            stop_time: None,
            restart_time: None,
        }
    }

    pub fn update_sunray(&mut self) {
        self.update_event(true);
    }

    pub fn update_asteroid(&mut self) {
        self.update_event(false);
    }

    fn update_event(&mut self, is_sunray: bool) {
        // Always decay for events
        self.update_no_event(true);

        // Add impulse for the current event
        if is_sunray {
            self.sun_intensity += self.impulse;
        } else {
            self.asteroid_intensity += self.impulse;
        }

        // Update probability
        self.update_probability();
    }

    fn update_no_event(&mut self, force_decay: bool) {
        let now = Instant::now();
        if let Some(last) = self.last_update {
            let elapsed = now.duration_since(last);

            if !force_decay && elapsed < self.min_time_constant {
                return;
            }

            let dt = elapsed.as_secs_f32();
            let tau = self.half_life.as_secs_f32() / std::f32::consts::LN_2;
            let decay_factor = (-dt / tau).exp();

            self.sun_intensity *= decay_factor;
            self.asteroid_intensity *= decay_factor;

            self.last_update = Some(now);

            if force_decay {
                self.update_probability();
            }
        } else {
            self.last_update = Some(now);
        }
    }

    fn update_probability(&mut self) {
        let s = self.sun_intensity + self.asteroid_intensity;
        self.sunray_probability = if s > 0.0 { self.sun_intensity / s } else { 0.5 };
    }

    pub fn restart(&mut self) {
        self.restart_time = Some(Instant::now());

        // Adjust last_update to account for the time spent stopped
        if let (Some(stop), Some(restart), Some(last)) =
            (self.stop_time, self.restart_time, self.last_update)
        {
            let stopped_duration = restart.duration_since(stop);
            self.last_update = Some(last + stopped_duration);
        }
    }

    pub fn stop(&mut self) {
        self.stop_time = Some(Instant::now());
    }

    pub fn sunray_probability(&mut self) -> f32 {
        self.update_no_event(false);
        self.sunray_probability
    }

    /*pub fn current_tau(&self) -> f32 {
        self.half_life.as_secs_f32() / std::f32::consts::LN_2
    }

    pub fn debug_stats(&self) -> (f32, f32) {
        (self.sun_intensity, self.asteroid_intensity)
    }*/
}
