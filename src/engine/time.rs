pub struct Time {
    sdl_timer: sdl2::TimerSubsystem,
    old_perf_counter: u64,
    perf_counter: u64,
    perf_freq: f64,
    delta_ms: f64,
    delta_s: f64,
    fps: f64,
}

impl Time {
    pub(in crate::engine) fn new(sdl_timer: sdl2::TimerSubsystem) -> Self {
        let perf_freq = sdl_timer.performance_frequency() as f64;
        Self {
            sdl_timer,
            old_perf_counter: 0,
            perf_counter: 0,
            perf_freq,
            delta_ms: 0f64,
            delta_s: 0f64,
            fps: f64::INFINITY,
        }
    }

    pub(in crate::engine) fn update(&mut self) {
        self.old_perf_counter = self.perf_counter;
        self.perf_counter = self.sdl_timer.performance_counter();
        self.delta_ms =
            ((self.perf_counter - self.old_perf_counter) * 1000) as f64 / self.perf_freq;
        self.delta_s = self.delta_ms / 1000f64;
        self.fps = 1f64 / self.delta_ms / 1000f64;
    }

    pub fn get_delta(&self) -> f64 {
        self.delta_s
    }

    pub fn get_delta_ms(&self) -> f64 {
        self.delta_ms
    }

    pub fn get_delta_fps(&self) -> f64 {
        self.fps
    }
}
