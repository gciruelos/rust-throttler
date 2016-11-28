//! A simple throttler library.
//! 
//! # Examples
//! 
//! The following example creates a throttler that allows 4 operations every 
//! second, and uses it to print the time 10 times.
//! 
//! ```
//! use throttler::simple::SimpleThrottler;
//! use std::time::{Duration, Instant};
//! 
//! let mut t = SimpleThrottler::new(4, Duration::new(1, 0));
//! 
//! for i in 0..10 {
//!     t.wait();
//!     println!("{} {:?}", i, Instant::now());
//! }
//! ```
//! 

pub mod simple;

#[cfg(test)]
mod tests {
    use simple::SimpleThrottler;
    use std::time::Duration;
    use std::time::Instant;

    #[test]
    fn simple_test() {
        let mut s = SimpleThrottler::new(100, Duration::new(1, 0));

        for i in 0..300 {
            s.wait();
            println!("{} {:?}", i, Instant::now());
        }
    }
}
