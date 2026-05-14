use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Closed,
    Open,
    HalfOpen,
}

struct Inner {
    state: State,
    failures: u32,
    fail_threshold: u32,
    cooldown: Duration,
    slow_start_gap: Duration,
    last_fail_time: Option<Instant>,
    last_probe_time: Option<Instant>,
    total_trips: u32,
}

/// Circuit breaker: pauses requests after consecutive failures.
/// Closed -> Open (after threshold) -> HalfOpen (after cooldown) -> Closed (on success).
pub struct CircuitBreaker {
    inner: Mutex<Inner>,
}

impl CircuitBreaker {
    pub fn new(fail_threshold: u32, cooldown: Duration) -> Self {
        Self {
            inner: Mutex::new(Inner {
                state: State::Closed,
                failures: 0,
                fail_threshold,
                cooldown,
                slow_start_gap: Duration::from_secs(2),
                last_fail_time: None,
                last_probe_time: None,
                total_trips: 0,
            }),
        }
    }

    /// Returns Ok(()) if a request is permitted, Err with wait description if not.
    pub fn allow(&self) -> Result<(), String> {
        let mut s = self.inner.lock().unwrap();
        match s.state {
            State::Closed => Ok(()),
            State::HalfOpen => {
                if let Some(probe_time) = s.last_probe_time {
                    if probe_time.elapsed() < s.slow_start_gap {
                        let remaining = s.slow_start_gap - probe_time.elapsed();
                        return Err(format!(
                            "reconnecting, next attempt in {:?}",
                            remaining
                        ));
                    }
                }
                s.last_probe_time = Some(Instant::now());
                Ok(())
            }
            State::Open => {
                if let Some(fail_time) = s.last_fail_time {
                    if fail_time.elapsed() > s.cooldown {
                        s.state = State::HalfOpen;
                        s.last_probe_time = Some(Instant::now());
                        log::info!("Reconnecting...");
                        return Ok(());
                    }
                    let remaining = s.cooldown - fail_time.elapsed();
                    Err(format!(
                        "service paused ({} pauses, {:?} remaining)",
                        s.total_trips, remaining
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn record_success(&self) {
        let mut s = self.inner.lock().unwrap();
        if s.state == State::HalfOpen {
            log::info!("Connection restored");
        }
        s.failures = 0;
        s.state = State::Closed;
    }

    /// Returns true if the breaker just tripped open.
    pub fn record_failure(&self) -> bool {
        let mut s = self.inner.lock().unwrap();
        s.failures += 1;
        s.last_fail_time = Some(Instant::now());

        if s.failures >= s.fail_threshold && s.state != State::Open {
            s.state = State::Open;
            s.total_trips += 1;
            log::warn!("Paused after {} errors", s.failures);
            return true;
        }

        if s.state == State::HalfOpen {
            s.state = State::Open;
            s.total_trips += 1;
            log::warn!("Reconnection failed, retrying...");
            return true;
        }

        false
    }

    pub fn trips(&self) -> u32 {
        self.inner.lock().unwrap().total_trips
    }

    pub fn failures(&self) -> u32 {
        self.inner.lock().unwrap().failures
    }

    pub fn reset_failures(&self) {
        self.inner.lock().unwrap().failures = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breaker_opens_after_threshold() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(30));
        assert!(cb.allow().is_ok());
        assert!(!cb.record_failure()); // 1
        assert!(!cb.record_failure()); // 2
        assert!(cb.record_failure());  // 3 -> trips
        assert!(cb.allow().is_err());  // blocked
    }

    #[test]
    fn breaker_trips_on_threshold() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(30));
        assert!(!cb.record_failure()); // 1
        assert!(!cb.record_failure()); // 2
        assert!(cb.record_failure());  // 3 -> trips
        assert!(cb.allow().is_err());
    }

    #[test]
    fn breaker_resets_on_success() {
        let cb = CircuitBreaker::new(3, Duration::from_millis(10));
        cb.record_failure();
        cb.record_failure();
        cb.record_failure();
        // Wait for cooldown
        std::thread::sleep(Duration::from_millis(15));
        assert!(cb.allow().is_ok()); // half-open probe
        cb.record_success();
        assert!(cb.allow().is_ok()); // closed again
        assert_eq!(cb.failures(), 0);
    }
}
