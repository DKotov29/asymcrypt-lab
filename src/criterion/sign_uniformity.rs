use std::time::{Duration, Instant};
use distrs::Normal;

pub fn test(seq: &[u8], a: f64) -> (bool, f64, f64, Duration) {
    let start_time = Instant::now();
    let mut statistic_value = 0f64;
    let len = seq.len();
    let nj = len as f64 / 256f64;
    let mut freq = [0usize; 256];
    for i in seq.iter() {
        freq[*i as usize] += 1;
    }
    for j in 0..=255u8 {
        let aa = freq[j as usize] as f64 - nj;
        statistic_value += aa.powf(2f64) / nj;
    }
    let l = 255f64;

    let z = Normal::ppf(1.0 - a, 0f64, 1f64);
    let critic_value: f64 = (2f64 * l).sqrt() * z + l;
    let elapsed = start_time.elapsed();
    (
        statistic_value <= critic_value,
        statistic_value,
        critic_value,
        elapsed,
    )
}
