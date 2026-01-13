/// Audio mixing operations

/// Apply volume scaling to a sample
pub fn apply_volume(sample: i16, volume: f64) -> i16 {
    ((sample as f64) * volume) as i16
}

/// Apply fade to samples
#[allow(dead_code)]
pub fn apply_fade(
    samples: &[i16],
    fade_in_samples: usize,
    fade_out_samples: usize,
) -> Vec<i16> {
    if fade_in_samples == 0 && fade_out_samples == 0 {
        return samples.to_vec();
    }

    let mut result = samples.to_vec();
    let length = result.len();

    // Apply fade in
    let fade_in_end = fade_in_samples.min(length);
    for i in 0..fade_in_end {
        result[i] = ((result[i] as f64) * (i as f64 / fade_in_samples as f64)) as i16;
    }

    // Apply fade out
    let fade_out_start = length.saturating_sub(fade_out_samples);
    for i in fade_out_start..length {
        let fade_out_index = length - 1 - i;
        result[i] = ((result[i] as f64) * (fade_out_index as f64 / fade_out_samples as f64)) as i16;
    }

    result
}

/// Mix two sample arrays together with clipping protection
#[allow(dead_code)]
pub fn mix_samples(samples1: &[i16], samples2: &[i16]) -> Vec<i16> {
    let max_length = samples1.len().max(samples2.len());
    let mut result = Vec::with_capacity(max_length);

    for i in 0..max_length {
        let s1 = if i < samples1.len() { samples1[i] } else { 0 };
        let s2 = if i < samples2.len() { samples2[i] } else { 0 };

        let mixed = (s1 as i32 + s2 as i32).clamp(-32768, 32767);
        result.push(mixed as i16);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_volume() {
        assert_eq!(apply_volume(1000, 0.5), 500);
        assert_eq!(apply_volume(1000, 1.0), 1000);
        assert_eq!(apply_volume(1000, 0.0), 0);
    }

    #[test]
    fn test_mix_samples() {
        let s1 = vec![1000, 2000, 3000];
        let s2 = vec![500, 1000, 1500];
        let mixed = mix_samples(&s1, &s2);
        assert_eq!(mixed, vec![1500, 3000, 4500]);
    }
}
