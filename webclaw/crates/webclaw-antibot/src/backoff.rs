use rand::Rng;
use std::time::Duration;

/// Randomized exponential backoff: uniform in [0, min(base * 2^attempt, max)].
pub fn backoff_delay(attempt: u32, base: Duration, max: Duration) -> Duration {
    let exp = 2f64.powi(attempt as i32);
    let ceiling = Duration::from_secs_f64(base.as_secs_f64() * exp).min(max);
    let ms = rand::thread_rng().gen_range(0..=ceiling.as_millis() as u64);
    Duration::from_millis(ms)
}

/// Poisson-distributed delay clamped to [min_floor, 3 * mean].
pub fn poisson_delay(mean: Duration, min_floor: Duration) -> Duration {
    let u: f64 = rand::thread_rng().gen_range(0.001..1.0);
    let delay_secs = -mean.as_secs_f64() * u.ln();
    let delay = Duration::from_secs_f64(delay_secs);
    let max = mean * 3;
    delay.max(min_floor).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_respects_max() {
        for attempt in 0..20 {
            let d = backoff_delay(attempt, Duration::from_millis(100), Duration::from_secs(30));
            assert!(d <= Duration::from_secs(30));
        }
    }

    #[test]
    fn poisson_respects_bounds() {
        for _ in 0..100 {
            let d = poisson_delay(Duration::from_millis(300), Duration::from_millis(50));
            assert!(d >= Duration::from_millis(50));
            assert!(d <= Duration::from_millis(900));
        }
    }
}
