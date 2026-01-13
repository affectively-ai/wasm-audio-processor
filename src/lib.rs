use wasm_bindgen::prelude::*;
use base64::{Engine as _, engine::general_purpose};

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

mod mulaw;
mod mixer;

use mulaw::{mu_law_decode, mu_law_encode};
use mixer::apply_volume;

/// Audio mixing configuration
#[wasm_bindgen]
pub struct AudioMixerConfig {
    whisper_volume: f64,
    original_volume: f64,
    fade_in_ms: f64,
    fade_out_ms: f64,
    sample_rate: f64,
}

#[wasm_bindgen]
impl AudioMixerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        whisper_volume: f64,
        original_volume: f64,
        fade_in_ms: f64,
        fade_out_ms: f64,
        sample_rate: f64,
    ) -> AudioMixerConfig {
        AudioMixerConfig {
            whisper_volume,
            original_volume,
            fade_in_ms,
            fade_out_ms,
            sample_rate,
        }
    }
}

/// Mix two audio streams together
/// 
/// # Arguments
/// * `original_audio` - Base64-encoded original mu-law audio
/// * `whisper_audio` - Base64-encoded whisper mu-law audio
/// * `config` - Mixing configuration
/// 
/// # Returns
/// Base64-encoded mixed audio
#[wasm_bindgen]
pub fn mix_audio_streams(
    original_audio: &str,
    whisper_audio: &str,
    config: &AudioMixerConfig,
) -> String {
    // Decode base64 strings to bytes
    let original_bytes = base64_decode(original_audio);
    let whisper_bytes = base64_decode(whisper_audio);

    // Calculate fade samples
    let fade_in_samples = ((config.fade_in_ms / 1000.0) * config.sample_rate) as usize;
    let fade_out_samples = ((config.fade_out_ms / 1000.0) * config.sample_rate) as usize;

    // Decode mu-law to linear samples (pre-allocate for better performance)
    let mut original_samples = Vec::with_capacity(original_bytes.len());
    for &b in &original_bytes {
        original_samples.push(mu_law_decode(b));
    }

    let mut whisper_samples = Vec::with_capacity(whisper_bytes.len());
    for &b in &whisper_bytes {
        whisper_samples.push(mu_law_decode(b));
    }

    // Apply volume scaling and fade to whisper (pre-allocate for better performance)
    let mut scaled_whisper = Vec::with_capacity(whisper_samples.len());
    let whisper_len = whisper_samples.len();
    for (i, &sample) in whisper_samples.iter().enumerate() {
        let mut scaled = apply_volume(sample, config.whisper_volume);
        
        // Apply fade in
        if i < fade_in_samples && fade_in_samples > 0 {
            scaled = (scaled as f64 * (i as f64 / fade_in_samples as f64)) as i16;
        }
        // Apply fade out
        else if i >= whisper_len - fade_out_samples && fade_out_samples > 0 {
            let fade_out_index = whisper_len - 1 - i;
            scaled = (scaled as f64 * (fade_out_index as f64 / fade_out_samples as f64)) as i16;
        }
        
        scaled_whisper.push(scaled);
    }

    // Scale original if needed (pre-allocate for better performance)
    let scaled_original: Vec<i16> = if config.original_volume != 1.0 {
        let mut scaled = Vec::with_capacity(original_samples.len());
        for &s in &original_samples {
            scaled.push(apply_volume(s, config.original_volume));
        }
        scaled
    } else {
        original_samples
    };

    // Mix samples (pre-allocate for better performance)
    let max_length = scaled_original.len().max(scaled_whisper.len());
    let mut mixed_samples = Vec::with_capacity(max_length);
    for i in 0..max_length {
        let original = if i < scaled_original.len() {
            scaled_original[i]
        } else {
            0
        };
        let whisper = if i < scaled_whisper.len() {
            scaled_whisper[i]
        } else {
            0
        };

        // Mix with clipping protection
        let mixed = original as i32 + whisper as i32;
        mixed_samples.push(mixed.clamp(-32768, 32767) as i16);
    }

    // Encode back to mu-law (pre-allocate for better performance)
    let mut mixed_bytes = Vec::with_capacity(mixed_samples.len());
    for &s in &mixed_samples {
        mixed_bytes.push(mu_law_encode(s));
    }

    // Encode to base64
    base64_encode(&mixed_bytes)
}

/// Reduce volume of audio
#[wasm_bindgen]
pub fn reduce_volume(audio: &str, volume: f64) -> String {
    let bytes = base64_decode(audio);
    
    // Decode mu-law (pre-allocate for better performance)
    let mut samples = Vec::with_capacity(bytes.len());
    for &b in &bytes {
        samples.push(mu_law_decode(b));
    }
    
    // Apply volume (pre-allocate for better performance)
    let mut scaled = Vec::with_capacity(samples.len());
    for &s in &samples {
        scaled.push(apply_volume(s, volume));
    }
    
    // Encode back to mu-law (pre-allocate for better performance)
    let mut output = Vec::with_capacity(scaled.len());
    for &s in &scaled {
        output.push(mu_law_encode(s));
    }
    
    base64_encode(&output)
}

/// Create silence buffer
#[wasm_bindgen]
pub fn create_silence(duration_ms: f64, sample_rate: f64) -> String {
    let num_samples = ((duration_ms / 1000.0) * sample_rate) as usize;
    let silence = vec![0xffu8; num_samples]; // 0xff is silence in mu-law
    base64_encode(&silence)
}

// Helper functions for base64 encoding/decoding
fn base64_decode(input: &str) -> Vec<u8> {
    general_purpose::STANDARD
        .decode(input)
        .unwrap_or_default()
}

fn base64_encode(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}
