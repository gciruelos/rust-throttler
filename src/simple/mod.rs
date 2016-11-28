use std::{thread, time};

pub struct SimpleThrottler {
    times_: u64,
    interval_: time::Duration,

    times_left_: u64,
    last_interval_: time::Instant,
}

impl SimpleThrottler {
    pub fn new(times: u64, interval: time::Duration) -> SimpleThrottler {
        SimpleThrottler {
            times_: times,
            interval_: interval,

            times_left_: times,
            last_interval_: time::Instant::now(),
        }
    }
    pub fn wait(&mut self) {
        let curr_interval = time::Instant::now().duration_since(self.last_interval_);
        if curr_interval > self.interval_ {
            self.restart();
        } else if self.times_left_ == 0 {
            thread::sleep(self.interval_ - curr_interval);
            self.restart();
        }
        self.times_left_ -= 1;
    }

    pub fn restart(&mut self) {
        self.times_left_ = self.times_;
        self.last_interval_ = time::Instant::now()
    }
}
