use time::Instant;

pub struct RateCounter {
    samples: Vec<Instant>,
    index: usize,
}

impl RateCounter {
    pub fn with_capacity(capacity: usize) -> Self {
        RateCounter {
            samples: Vec::with_capacity(capacity),
            index: 0,
        }
    }

    pub fn update(&mut self) -> f64 {
        let now = Instant::now();
        let then;
        if self.samples.len() < self.samples.capacity() {
            // While we have room, just push samples and samples[0] is
            // the first sample.
            self.samples.push(now);
            then = self.samples[0];
        } else {
            // Take the old sample, overwrite it and circularly
            // increment index.
            then = self.samples[self.index];
            self.samples[self.index] = now;
            self.index = if self.index + 1 < self.samples.len() {
                self.index + 1
            } else {
                0
            };
        }

        // Will return NaN when now - then is zero which will happen at
        // least when samples.len() == 1.
        let elapsed = now - then;
        let updates_per_second = (self.samples.len() as u64 * 1_000_000_000) as f64
            / (elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64) as f64;
        updates_per_second
    }
}
