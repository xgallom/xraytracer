use std::time::{Duration, Instant};

const TIMES_COUNT: usize = 6;

pub struct Time {
    size: usize,
    times: [Instant; TIMES_COUNT],
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            size: 0,
            times: [now; TIMES_COUNT],
        }
    }

    pub fn record(&mut self) -> Duration {
        let now = Instant::now();
        let before = self.times[self.size];
        self.size += 1;
        self.times[self.size] = now;

        now.duration_since(before)
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.times[0])
    }
}
