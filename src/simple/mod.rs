use std::{thread, time};

pub struct SimpleThrottler {
    times: u64,
    interval: time::Duration,

    times_left: u64,
    last_interval: time::Instant,
}

impl SimpleThrottler {
    /// Constructs a new `SimpleThrottler`. Takes as parameters the number of
    /// operations allowed per time interval, and the length of the interval.
    /// 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use throttler::simple::SimpleThrottler;
    /// use std::time::Duration;
    /// 
    /// let throttler = SimpleThrottler::new(1000, Duration::new(1, 0));
    /// ```
    pub fn new(operations_per_interval: u64, time_interval: time::Duration)
        -> SimpleThrottler {
        SimpleThrottler {
            times: operations_per_interval,
            interval: time_interval,

            times_left: operations_per_interval,
            last_interval: time::Instant::now(),
        }
    }

    /// Waits until the throttler allows to make more operations. If no more
    /// operations are allowed in the current time interval, the throttler
    /// waits for it to end.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use throttler::simple::SimpleThrottler;
    /// use std::time::Duration;
    /// 
    /// let mut throttler = SimpleThrottler::new(3, Duration::new(1, 0));
    /// // This will print all numbers from 0 to 10 in bursts of 3.
    /// for i in 0..10 {
    ///     throttler.wait();
    ///     println!("{}", i);
    /// }
    /// ```
    pub fn wait(&mut self) {
        let curr_interval = time::Instant::now()
            .duration_since(self.last_interval);
        if curr_interval > self.interval {
            self.restart();
        } else if self.times_left == 0 {
            thread::sleep(self.interval - curr_interval);
            self.restart();
        }
        self.times_left -= 1;
    }

    /// Restarts the throttler completely. After calling `restart` the throttler
    /// is restored to its initial state, like after the `new` call.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use throttler::simple::SimpleThrottler;
    /// use std::time::Duration;
    /// 
    /// let mut throttler = SimpleThrottler::new(3, Duration::new(1, 0));
    /// // This will print all numbers from 0 to 10 together, because the
    /// // throttler always gets restarted.
    /// for i in 0..10 {
    ///     throttler.wait();
    ///     println!("{}", i);
    ///     throttler.restart()
    /// }
    /// ```
    pub fn restart(&mut self) {
        self.times_left = self.times;
        self.last_interval = time::Instant::now()
    }
}
