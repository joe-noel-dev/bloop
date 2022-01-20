#[derive(Default)]
pub struct PeriodicNotification {
    interval_samples: i64,
    countdown: i64,
}

impl PeriodicNotification {
    pub fn reset(&mut self, sample_rate: u32, interval_hz: f64) {
        self.interval_samples = (sample_rate as f64 / interval_hz) as i64;
        self.countdown = 0;
    }

    pub fn increment(&mut self, num_samples: i64) -> bool {
        let mut should_notify = false;

        self.countdown -= num_samples;

        if self.countdown < 0 {
            should_notify = true;
        }

        while self.countdown < 0 {
            self.countdown += self.interval_samples;
        }

        should_notify
    }
}
