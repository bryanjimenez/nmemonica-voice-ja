/// Trim wave start and end below threshold points using a cutoff
pub fn trim_wave(wave: Vec<f64>, cutoff: f64, end_padding: Option<usize>) -> Vec<f64> {
    // let before = wave.len();
    // println!("samples: {}", before);

    let mut start = 0;
    let mut end = wave.len();
    for (i, sampl) in wave.iter().enumerate() {
        if start == 0 && (*sampl).abs() > cutoff {
            start = i;
        }

        if start > 0 && (*sampl).abs() > cutoff {
            end = i;
        }
    }

    // println!("start: {start} end: {end}");

    // if start - padding > 0 {
    //     start -= padding;
    // }

    if let Some(padding) = end_padding {
        if end + padding < wave.len() {
            end += padding;
        }
    }

    // println!("start: {start} end: {end}");

    let wave: Vec<f64> = wave
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if i >= start && i <= end {
                return Some(v);
            }

            None
        })
        .collect();

    // let after = wave.len();
    // let after_percent: f64 = (after as f64 / before as f64) * 100.0;

    // println!("samples: {} {:.2}%", after, after_percent);
    wave
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_wave_fn() {
        let wave: [f64; 10] = [
            0f64, 0_f64, 0_f64, 600_f64, 600_f64, 600_f64, 0_f64, 600_f64, 0_f64, 0f64,
        ];
        let a = trim_wave(wave.to_vec(), 500_f64, None);

        assert_eq!(a, [600_f64, 600_f64, 600_f64, 0_f64, 600_f64]);
    }
}
