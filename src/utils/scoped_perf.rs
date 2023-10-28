use pprof::ProfilerGuard;

pub struct ScopedPerf<'a> {
    p_guard: ProfilerGuard<'a>,
}

impl<'a> ScopedPerf<'a> {
    pub fn new() -> Self {
        let p_guard = pprof::ProfilerGuardBuilder::default()
            .frequency(10000)
            .build()
            .unwrap();
        Self { p_guard }
    }
}

impl<'a> Drop for ScopedPerf<'a> {
    fn drop(&mut self) {
        if let Ok(report) = self.p_guard.report().build() {
            let file = std::fs::File::create("data/perf_flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
        } else {
            panic!("Failed to create perf graph");
        }
    }
}
