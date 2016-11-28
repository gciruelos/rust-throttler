pub mod simple;


#[cfg(test)]
mod tests {
    use simple::SimpleThrottler;
    use std::time::Duration;

    #[test]
    fn simple_test() {
        let mut s = SimpleThrottler::new(1000, Duration::new(1, 0));
        let mut i = 0;

        while i < 10000 {
            s.wait();
            println!("{}", i);
            i += 1;
        }
    }
}
