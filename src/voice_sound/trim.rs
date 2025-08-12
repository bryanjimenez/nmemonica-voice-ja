/// Returns a trimmed wave that has a sub slice with start and endpoint above `cutoff`
///
/// If `end_padding` is specified the endpoint is extended accordingly
pub fn trim_wave<P: AsRef<[f64]>>(wave: P, cutoff: f64, end_padding: Option<usize>) -> Vec<f64> {
    let wave = wave.as_ref();

    // need to look ahead to find farthest endpoint
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

    if let Some(padding) = end_padding {
        if end + padding < wave.len() {
            end += padding;
        } else {
            end = wave.len() - 1;
        }
    }

    wave[start..=end].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_wave_fn() {
        let wave: [f64; 10] = [
            0f64, 0_f64, 0_f64, 600_f64, 600_f64, 600_f64, 0_f64, 600_f64, 0_f64, 0f64,
        ];
        let a = trim_wave(wave, 500_f64, None);

        assert_eq!(a, [600_f64, 600_f64, 600_f64, 0_f64, 600_f64]);
    }
}
