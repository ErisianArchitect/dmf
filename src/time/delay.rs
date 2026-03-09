use std::time::{Duration, Instant};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Delay {
    deadline: Instant,
}

impl Delay {
    #[inline]
    pub fn is_ready(self) -> bool {
        Instant::now() >= self.deadline
    }

    #[inline]
    pub fn deadline(self) -> Instant {
        self.deadline
    }

    #[inline]
    pub fn until(until: Instant) -> Self {
        Self { deadline: until }
    }
    
    #[inline]
    pub fn after_now(duration: Duration) -> Self {
        Self::until(Instant::now() + duration)
    }

    #[inline]
    pub fn nanos(nanos: u64) -> Self {
        Self::after_now(Duration::from_nanos(nanos))
    }

    #[inline]
    pub fn micros(micros: u64) -> Self {
        Self::after_now(Duration::from_micros(micros))
    }

    #[inline]
    pub fn millis(millis: u64) -> Self {
        Self::after_now(Duration::from_millis(millis))
    }

    #[inline]
    pub fn secs(secs: u64) -> Self {
        Self::after_now(Duration::from_secs(secs))
    }

    #[inline]
    pub fn secs_f32(secs: f32) -> Self {
        Self::after_now(Duration::from_secs_f32(secs))
    }

    #[inline]
    pub fn secs_f64(secs: f64) -> Self {
        Self::after_now(Duration::from_secs_f64(secs))
    }

    #[inline]
    pub fn mins(mins: u64) -> Self {
        Self::after_now(Duration::from_secs(mins * 60))
    }

    #[inline]
    pub fn mins_f32(mins: f32) -> Self {
        Self::after_now(Duration::from_secs_f32(mins * 60.0))
    }

    #[inline]
    pub fn mins_f64(mins: f64) -> Self {
        Self::after_now(Duration::from_secs_f64(mins * 60.0))
    }

    #[inline]
    pub fn hours(hours: u64) -> Self {
        Self::secs(hours * 3600)
    }

    #[inline]
    pub fn hours_f32(hours: f32) -> Self {
        Self::secs_f32(hours * 3600.0)
    }

    #[inline]
    pub fn hours_f64(hours: f64) -> Self {
        Self::secs_f64(hours * 3600.0)
    }

    #[inline]
    pub fn days(days: u64) -> Self {
        Self::secs(days * 86400)
    }

    #[inline]
    pub fn days_f32(days: f32) -> Self {
        Self::secs_f32(days * 86400.0)
    }

    #[inline]
    pub fn days_f64(days: f64) -> Self {
        Self::secs_f64(days * 86400.0)
    }
}