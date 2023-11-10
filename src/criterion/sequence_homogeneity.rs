use statrs::function::erf::erf_inv;
use std::time::{Duration, Instant};

pub fn test(seq: Vec<u8>, r: usize/*те на скільки розбиваємо*/, a: f64) -> (bool, f64, f64, Duration) {
    let start_time = Instant::now();
    let m = seq.len();
    let m1 = m / r;
    let n = m1 * r;

    let mut v_ij = Vec::with_capacity(r);
    (0..r).for_each(|_| v_ij.push([0usize; 256]));
    for (i, arr) in seq.chunks(m1).enumerate().take(m1) // r ітерацій
    {
        for arr_i in arr {
            v_ij[i][(*arr_i) as usize] += 1;
        }
    }
    let mut statistic_value = 0f64;
    for i in 0..=255 {
        for j in 0..=(r - 1) {
            let mut v_i = 0f64;
            for jj in 0..=(r - 1) {
                v_i += v_ij[jj][i] as f64;
            }
            statistic_value += ((*v_ij[j].get(i).unwrap() as f64).powf(2.0) / (m1 as f64 * v_i));
        }
    }
    statistic_value -= 1.0;
    statistic_value *= n as f64;

    let l = 255f64 * (r as f64 - 1f64);
    let z = erf_inv(1.0 - a);
    let critic_value: f64 = (2f64 * l).sqrt() * z + l;
    let elapsed = start_time.elapsed();
    return (
        statistic_value <= critic_value,
        statistic_value,
        critic_value,
        elapsed,
    );
}
