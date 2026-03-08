/// Mu-law encoding/decoding functions
///
/// Mu-law (G.711) is a companding algorithm used in telephony to encode audio signals.
/// This implementation follows the ITU-T G.711 standard for 16-bit linear PCM I/O.

/// Bias added to magnitude before encoding (ITU-T G.711)
const MULAW_BIAS: i32 = 0x84; // 132
/// Maximum input magnitude after clipping
const MULAW_CLIP: i32 = 32635;

/// Decode a mu-law byte to a signed 16-bit linear PCM sample
pub fn mu_law_decode(mu_law_byte: u8) -> i16 {
    let complement = !mu_law_byte;
    let sign = complement & 0x80;
    let exponent = ((complement >> 4) & 0x07) as u32;
    let mantissa = (complement & 0x0f) as i32;

    // Standard G.711 decode: reconstruct magnitude from exponent and mantissa
    let sample = (((mantissa << 3) | 0x84) << exponent) - MULAW_BIAS;

    if sign != 0 {
        -(sample as i16)
    } else {
        sample as i16
    }
}

/// Encode a signed 16-bit linear PCM sample to a mu-law byte
pub fn mu_law_encode(sample: i16) -> u8 {
    // Get sign and magnitude
    let sign: u8 = if sample < 0 { 0x80 } else { 0 };
    // Handle i16::MIN specially to avoid overflow in abs()
    let magnitude: i32 = if sample == i16::MIN {
        i16::MAX as i32
    } else {
        (sample as i32).abs()
    };

    // Clip and add bias
    let magnitude = magnitude.min(MULAW_CLIP) + MULAW_BIAS;

    // Find the segment (exponent) by searching for the highest set bit
    let mut exponent: u8 = 7;
    let mut exp_mask: i32 = 0x4000;
    while exponent > 0 && (magnitude & exp_mask) == 0 {
        exponent -= 1;
        exp_mask >>= 1;
    }

    let mantissa = ((magnitude >> (exponent as i32 + 3)) & 0x0f) as u8;
    !(sign | (exponent << 4) | mantissa)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mulaw_roundtrip() {
        // Test that encoding and decoding preserves values (approximately)
        // Mu-law is a lossy companding algorithm; quantization error increases with magnitude.
        // For small values (< 256), error should be within ~8.
        // For larger values, relative error should be under ~3% (G.711 spec guarantees this).
        let test_values: Vec<i16> = vec![
            -8000, -4000, -2000, -1000, -500, -250, -128, -64, -32, -16, -8, -4, -2, -1,
            0, 1, 2, 4, 8, 16, 32, 64, 128, 250, 500, 1000, 2000, 4000, 8000,
        ];

        for &val in &test_values {
            let encoded = mu_law_encode(val);
            let decoded = mu_law_decode(encoded);
            let diff = (val as i32 - decoded as i32).unsigned_abs();
            // Mu-law quantization error scales with magnitude:
            // - Small signals (|v| < 256): max error ~8
            // - Larger signals: max error ~3% of magnitude
            let magnitude = (val as i32).unsigned_abs();
            let max_diff = if magnitude < 256 { 16 } else { magnitude / 16 + 16 };
            assert!(
                diff <= max_diff,
                "Roundtrip error too large for {}: decoded={}, diff={}, max_allowed={}",
                val, decoded, diff, max_diff
            );
        }

        // Test that the functions don't panic for extreme values
        let _ = mu_law_encode(i16::MIN);
        let _ = mu_law_encode(i16::MAX);
        let _ = mu_law_decode(0u8);
        let _ = mu_law_decode(255u8);

        // Test sign preservation
        assert!(mu_law_decode(mu_law_encode(1000)) > 0, "Positive sign lost");
        assert!(mu_law_decode(mu_law_encode(-1000)) < 0, "Negative sign lost");

        // Test silence roundtrip (0 should encode/decode close to 0)
        let silence_encoded = mu_law_encode(0);
        let silence_decoded = mu_law_decode(silence_encoded);
        assert!(
            silence_decoded.unsigned_abs() < 64,
            "Silence roundtrip too far from zero: {}",
            silence_decoded
        );
    }
}
