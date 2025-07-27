mod delay;
use std::time::{Duration, Instant};

pub use delay::*;

#[derive(Debug)]
pub struct TimedResult<R> {
    pub result: R,
    pub elapsed: Duration,
}

impl<R> TimedResult<R> {
    #[must_use]
    #[inline(always)]
    fn new(result: R, elapsed: Duration) -> Self {
        Self { result, elapsed }
    }
}

#[must_use]
#[inline(always)]
pub fn time_it<R, F: FnOnce() -> R>(f: F) -> TimedResult<R> {
    let start_time = Instant::now();
    let result = f();
    let elapsed = start_time.elapsed();
    TimedResult::new(result, elapsed)
}