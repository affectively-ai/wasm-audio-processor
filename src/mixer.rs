/// Audio mixing operations

/// Buleyean weight: probability of sample contributing given rounds observed
/// and times rejected (silent). Samples silent in every round collapse to
/// floor weight and get skipped.
#[inline]
pub fn buleyean_weight(rounds: usize, rejections: usize) -> f64 {
    if rounds == 0 { return 1.0; }
    let r = rejections.min(rounds) as f64;
    let n = rounds as f64;
    (n - r) / n
}

/// Floor-weight threshold: samples at or below this are eliminated.
pub const FLOOR_WEIGHT: f64 = 0.0;

/// Apply volume scaling to a sample
pub fn apply_volume(sample: i16, volume: f64) -> i16 {
    ((sample as f64) * volume) as i16
}

/// Apply fade to samples
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

    // Apply fade in: ratio goes from near-zero at i=0 to 1.0 at i=fade_in_end
    if fade_in_samples > 0 {
        let fade_in_end = fade_in_samples.min(length);
        for i in 0..fade_in_end {
            result[i] = ((result[i] as f64) * ((i + 1) as f64 / (fade_in_samples + 1) as f64)) as i16;
        }
    }

    // Apply fade out: ratio goes from 1.0 down to near-zero at the last sample
    if fade_out_samples > 0 {
        let fade_out_start = length.saturating_sub(fade_out_samples);
        for i in fade_out_start..length {
            let samples_remaining = length - 1 - i;
            result[i] = ((result[i] as f64) * ((samples_remaining + 1) as f64 / (fade_out_samples + 1) as f64)) as i16;
        }
    }

    result
}

/// Mix two sample arrays together with clipping protection.
/// Deceptacon: skip floor-weight samples where both channels are silent.
pub fn mix_samples(samples1: &[i16], samples2: &[i16]) -> Vec<i16> {
    let max_length = samples1.len().max(samples2.len());
    let mut result = Vec::with_capacity(max_length);

    // Track consecutive silent samples for floor-weight elimination
    let mut silent_rounds: usize = 0;

    for i in 0..max_length {
        let s1 = if i < samples1.len() { samples1[i] } else { 0 };
        let s2 = if i < samples2.len() { samples2[i] } else { 0 };

        // Deceptacon: skip floor-weight samples.
        // If both channels are silent, track rejection rounds.
        if s1 == 0 && s2 == 0 {
            silent_rounds += 1;
            // After enough silent rounds, floor-weight samples are just silence
            if silent_rounds >= 10 && buleyean_weight(silent_rounds, silent_rounds) <= FLOOR_WEIGHT {
                result.push(0);
                continue;
            }
        } else {
            silent_rounds = 0;
        }

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
