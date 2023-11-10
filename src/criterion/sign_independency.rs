use statrs::function::erf::erf_inv;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn test(seq: Vec<u8>, a: f64) -> (bool, f64, f64, Duration) {
    let mut start_time = Instant::now();
    let n = seq.len() / 2;
    let mut freq: HashMap<(u8, u8), usize> = HashMap::new();
    let mut freq_first_elem = [0usize; 256];
    let mut freq_second_elem = [0usize; 256];
    let paiirs;
    {
        let mut pairs = seq.chunks(2);
        paiirs = pairs.take((n));
    }
    for x in paiirs {
        *freq.entry((x[0], x[1])).or_insert(0) += 1;
        freq_first_elem[x[0] as usize] += 1;
        freq_second_elem[x[1] as usize] += 1;
    }
    let mut statistic_value = 0f64;
    for i in 0..=255u8 {
        for j in 0..=255u8 {
            let o = ((*freq.get(&(i, j)).unwrap_or(&0)) as f64).powf(2.0);
            let v_i = freq_first_elem[i as usize];
            let a_j = freq_second_elem[j as usize];
            if v_i == 0 || a_j == 0 {
                continue;
            }
            let o = o / ((v_i * a_j) as f64);
            statistic_value += o;
        }
    }
    statistic_value -= 1.0;
    statistic_value *= n as f64;
    let l = 255f64.powf(2.0);
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
