use std::fmt;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct TimingResult {
    duration: Duration,
}

impl TimingResult {
    pub fn from_duration(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn get_guration(self) -> Duration {
        self.duration
    }
}

impl fmt::Display for TimingResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let duration = self.duration;
        let us = duration.as_micros();
        let ms = duration.as_millis();

        let time = ms as f64 + (us - ms * 1000) as f64 / 1000.0;
        f.pad(&format!("{:.2} {}", time, "ms"))
    }
}

pub struct Timing {
    start: Instant,
}

impl Timing {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn result(&self) -> TimingResult {
        TimingResult::from_duration(self.start.elapsed())
    }
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.result().fmt(f)
    }
}
