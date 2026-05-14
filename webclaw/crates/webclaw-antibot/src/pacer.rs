use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::backoff::poisson_delay;

#[derive(Debug, Clone)]
pub struct PacerConfig {
    pub initial_delay: Duration,
    pub min_delay: Duration,
    pub max_delay: Duration,
    pub window_size: usize,
    pub speed_up_rate: f64,
    pub slow_down_rate: f64,
}

impl Default for PacerConfig {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(300),
            min_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(8),
            window_size: 10,
            speed_up_rate: 0.9,
            slow_down_rate: 2.0,
        }
    }
}

struct Inner {
    results: Vec<bool>,
    pos: usize,
    filled: bool,
    mean_delay: Duration,
    min_delay: Duration,
    max_delay: Duration,
    speed_up_rate: f64,
    slow_down_rate: f64,

    latencies: Vec<Duration>,
    lat_pos: usize,
    lat_filled: bool,

    rate_limit_freeze_until: Option<Instant>,
}

/// Adaptive request pacer. Speeds up on sustained success, slows down on failure.
/// Uses a sliding window of results and latency spike detection.
pub struct AdaptivePacer {
    inner: Mutex<Inner>,
}

impl AdaptivePacer {
    pub fn new(cfg: PacerConfig) -> Self {
        Self {
            inner: Mutex::new(Inner {
                results: vec![false; cfg.window_size],
                pos: 0,
                filled: false,
                mean_delay: cfg.initial_delay,
                min_delay: cfg.min_delay,
                max_delay: cfg.max_delay,
                speed_up_rate: cfg.speed_up_rate,
                slow_down_rate: cfg.slow_down_rate,
                latencies: vec![Duration::ZERO; 10],
                lat_pos: 0,
                lat_filled: false,
                rate_limit_freeze_until: None,
            }),
        }
    }

    pub fn record_success(&self) {
        let mut s = self.inner.lock().unwrap();
        let pos = s.pos;
        s.results[pos] = true;
        s.pos = (s.pos + 1) % s.results.len();
        if s.pos == 0 {
            s.filled = true;
        }

        let frozen = s
            .rate_limit_freeze_until
            .map(|t| Instant::now() < t)
            .unwrap_or(false);

        if s.filled && success_rate(&s.results, s.filled, s.pos) >= 1.0 && !frozen {
            let new_delay =
                Duration::from_secs_f64(s.mean_delay.as_secs_f64() * s.speed_up_rate);
            let new_delay = new_delay.max(s.min_delay);
            if new_delay != s.mean_delay {
                s.mean_delay = new_delay;
                log::info!("Optimizing throughput (delay: {}ms)", s.mean_delay.as_millis());
            }
        }
    }

    pub fn record_failure(&self) {
        let mut s = self.inner.lock().unwrap();
        let pos = s.pos;
        s.results[pos] = false;
        s.pos = (s.pos + 1) % s.results.len();
        if s.pos == 0 {
            s.filled = true;
        }

        let new_delay =
            Duration::from_secs_f64(s.mean_delay.as_secs_f64() * s.slow_down_rate);
        s.mean_delay = new_delay.min(s.max_delay);
        log::warn!("Adjusting request rate (delay: {}ms)", s.mean_delay.as_millis());
    }

    /// Record latency. Returns true if a spike was detected (4x avg + low success).
    pub fn record_latency(&self, d: Duration) -> bool {
        let mut s = self.inner.lock().unwrap();
        let lat_pos = s.lat_pos;
        s.latencies[lat_pos] = d;
        s.lat_pos = (s.lat_pos + 1) % s.latencies.len();
        if s.lat_pos == 0 {
            s.lat_filled = true;
        }

        let avg = avg_latency(&s.latencies, s.lat_filled, s.lat_pos);
        if avg == Duration::ZERO {
            return false;
        }

        let rate = success_rate(&s.results, s.filled, s.pos);
        if d.as_secs_f64() > avg.as_secs_f64() * 4.0 && s.filled && rate < 0.7 {
            let new_delay =
                Duration::from_secs_f64(s.mean_delay.as_secs_f64() * 1.3);
            s.mean_delay = new_delay.min(s.max_delay);
            log::warn!("Latency spike detected, adjusting rate");
            return true;
        }
        false
    }

    pub fn current_delay(&self) -> Duration {
        self.inner.lock().unwrap().mean_delay
    }

    pub fn success_rate(&self) -> f64 {
        let s = self.inner.lock().unwrap();
        success_rate(&s.results, s.filled, s.pos)
    }

    pub fn avg_latency(&self) -> Duration {
        let s = self.inner.lock().unwrap();
        avg_latency(&s.latencies, s.lat_filled, s.lat_pos)
    }

    /// Block for current delay with Poisson jitter.
    pub fn wait(&self) {
        let delay = self.current_delay();
        let actual = poisson_delay(delay, Duration::from_millis(50));
        std::thread::sleep(actual);
    }

    /// Set a minimum delay floor (e.g. from Retry-After header). Frozen for 5 minutes.
    pub fn set_min_delay(&self, d: Duration) {
        let mut s = self.inner.lock().unwrap();
        if d > s.mean_delay {
            s.mean_delay = d.min(s.max_delay);
            s.rate_limit_freeze_until = Some(Instant::now() + Duration::from_secs(300));
        }
    }
}

fn success_rate(results: &[bool], filled: bool, pos: usize) -> f64 {
    let count = if filled { results.len() } else { pos };
    if count == 0 {
        return 1.0;
    }
    let successes = results[..count].iter().filter(|&&r| r).count();
    successes as f64 / count as f64
}

fn avg_latency(latencies: &[Duration], filled: bool, pos: usize) -> Duration {
    let count = if filled { latencies.len() } else { pos };
    if count == 0 {
        return Duration::ZERO;
    }
    let total: Duration = latencies[..count].iter().sum();
    total / count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pacer_speeds_up_on_success() {
        let p = AdaptivePacer::new(PacerConfig {
            initial_delay: Duration::from_millis(300),
            window_size: 5,
            speed_up_rate: 0.8,
            ..Default::default()
        });
        // Fill window with successes
        for _ in 0..10 {
            p.record_success();
        }
        assert!(p.current_delay() < Duration::from_millis(300));
    }

    #[test]
    fn pacer_slows_on_failure() {
        let p = AdaptivePacer::new(PacerConfig::default());
        let before = p.current_delay();
        p.record_failure();
        assert!(p.current_delay() > before);
    }

    #[test]
    fn pacer_respects_max() {
        let p = AdaptivePacer::new(PacerConfig {
            max_delay: Duration::from_secs(8),
            ..Default::default()
        });
        for _ in 0..20 {
            p.record_failure();
        }
        assert!(p.current_delay() <= Duration::from_secs(8));
    }
}
