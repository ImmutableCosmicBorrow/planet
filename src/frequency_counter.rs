use std::time::{Duration, Instant};

/// Bayesian estimator with fully adaptive τ based on observed event intervals,
/// volatility, and trend. No upper bound on τ.
pub struct FrequencyCounter {
    // Probability estimate
    p_sunray: f64,

    // Bayesian likelihoods
    L_sunray: f64,
    L_asteroid: f64,

    // Timing
    last_update: Option<Instant>,
    last_event_time: Option<Instant>,
    last_event_value: Option<f64>,

    // EWMA estimates
    ewma_interval: f64,        // estimated mean inter-event interval
    interval_smoothing: f64,   // alpha for EWMA of intervals
    volatility: f64,           // smoothed flip rate [0..1]
    volatility_smoothing: f64, // alpha for EWMA of flips
    interval_trend: f64,       // smoothed fractional interval change
    trend_smoothing: f64,      // alpha for trend

    // τ scaling factor
    tau_scale: f64,     // k in tau = k * ewma_interval * f_vol * f_trend
    tau_min: f64,       // minimal τ to avoid numerical issues
    tau: f64,           // current adaptive τ

    min_interval: f64,  // safety floor for interval EWMA
}

impl FrequencyCounter {
    /// Initialize a new FrequencyCounter
    pub fn new(
        tau_scale: f64,
        tau_min: f64,
        interval_smoothing: f64,
        volatility_smoothing: f64,
        trend_smoothing: f64,
        L_sunray: f64,
        L_asteroid: f64,
    ) -> Self {
        assert!(tau_min > 0.0);
        assert!(interval_smoothing > 0.0 && interval_smoothing < 1.0);
        assert!(volatility_smoothing > 0.0 && volatility_smoothing < 1.0);
        assert!(trend_smoothing > 0.0 && trend_smoothing < 1.0);
        assert!(L_asteroid > 0.0 && L_asteroid < L_sunray && L_sunray <= 1.0);

        let min_interval = 1e-3;

        Self {
            p_sunray: 0.8,
            L_sunray,
            L_asteroid,
            last_update: None,
            last_event_time: None,
            last_event_value: None,
            ewma_interval: 1.0,
            interval_smoothing,
            volatility: 0.0,
            volatility_smoothing,
            interval_trend: 0.0,
            trend_smoothing,
            tau_scale,
            tau_min,
            tau: tau_min,
            min_interval,
        }
    }

    pub fn update_sunray(&mut self) {
        self.update_event(true);
    }

    pub fn update_asteroid(&mut self) {
        self.update_event(false);
    }

    fn update_event(&mut self, is_sunray: bool) {
        let now = Instant::now();
        let event_value = if is_sunray { 1.0 } else { 0.0 };

        // 1) Update interval EWMA if we have a previous event
        if let Some(prev_time) = self.last_event_time {
            let dt = now.duration_since(prev_time).as_secs_f64().max(self.min_interval);

            // EWMA for interval
            self.ewma_interval = (1.0 - self.interval_smoothing) * self.ewma_interval
                + self.interval_smoothing * dt;

            // Trend: fractional change in interval
            let frac_change = (dt - self.ewma_interval) / self.ewma_interval;
            self.interval_trend = (1.0 - self.trend_smoothing) * self.interval_trend
                + self.trend_smoothing * frac_change;

            // Volatility: did label flip?
            if let Some(prev_val) = self.last_event_value {
                let v = (event_value - prev_val).abs();
                self.volatility = (1.0 - self.volatility_smoothing) * self.volatility
                    + self.volatility_smoothing * v;
            }
        }

        self.last_event_time = Some(now);
        self.last_event_value = Some(event_value);

        // 2) Compute adaptive τ
        let vol_strength = 0.8;
        let trend_strength = 2.0;

        let f_vol = 1.0 - (self.volatility * vol_strength);
        let f_trend = 1.0 - (self.interval_trend * trend_strength);

        let mut tau_candidate = self.tau_scale * self.ewma_interval * f_vol * f_trend;
        if !tau_candidate.is_finite() || tau_candidate < self.tau_min {
            tau_candidate = self.tau_min;
        }

        self.tau = tau_candidate;

        // 3) Apply decay using adaptive τ
        self.update_no_event_with_now(now);

        // 4) Bayesian posterior update
        let p = self.p_sunray;
        let L_s = if is_sunray { self.L_sunray } else { 1.0 - self.L_sunray };
        let L_a = if is_sunray { self.L_asteroid } else { 1.0 - self.L_asteroid };

        let numerator = p * L_s;
        let denominator = numerator + (1.0 - p) * L_a;
        if denominator > 0.0 {
            self.p_sunray = (numerator / denominator).clamp(0.0, 1.0);
        }
    }

    fn update_no_event(&mut self) {
        let now = Instant::now();
        self.update_no_event_with_now(now);
    }

    fn update_no_event_with_now(&mut self, now: Instant) {
        let dt = if let Some(last) = self.last_update {
            now.duration_since(last).as_secs_f64()
        } else {
            0.0
        };
        self.last_update = Some(now);

        if dt > 0.0 {
            let decay = (-dt / self.tau).exp();
            self.p_sunray *= decay;
        }
    }

    pub fn p_sunray(&mut self) -> f64 {
        self.update_no_event();
        self.p_sunray
    }

    pub fn current_tau(&self) -> f64 {
        self.tau
    }

    pub fn debug_stats(&self) -> (f64, f64, f64) {
        (self.ewma_interval, self.volatility, self.interval_trend)
    }
}
