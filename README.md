# rust-throttler
Simple throttler for Rust.

[![Crate Version](https://img.shields.io/crates/v/throttler.svg)](https://crates.io/crates/throttler)
[![Build Status](https://travis-ci.org/gciruelos/rust-throttler.svg?branch=master)](https://travis-ci.org/gciruelos/rust-throttler)
[![Documentation](https://docs.rs/throttler/badge.svg)](https://docs.rs/throttler)


It has just a really simple and easy to use throttler struct right now, but fancier ones will be added in the future.

## Examples


The following example creates a throttler that allows 4 operations every 
second, and uses it to print the time 10 times.

```rust
use throttler::simple::SimpleThrottler;
use std::time::{Duration, Instant};

let mut t = SimpleThrottler::new(4, Duration::new(1, 0));

for i in 0..10 {
    t.wait();
    println!("{} {:?}", i, Instant::now());
}
```

## Contributions

Feel free to contribute whatever you want.
