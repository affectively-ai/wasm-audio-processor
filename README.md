# @affectively/wasm-audio-processor

`@affectively/wasm-audio-processor` is a Rust/WebAssembly module for simple audio manipulation tasks such as mixing, volume adjustment, silence generation, and mu-law handling.

The fair brag is practicality. The package is narrow, but it covers a real set of tasks that show up in voice and telephony flows without asking the caller to build a full DSP stack first.

## What It Helps You Do

- mix two audio streams
- lower volume
- generate silence buffers
- work with mu-law encoded audio

## Installation

### npm

```bash
npm install @affectively/wasm-audio-processor
```

### Cargo

```toml
[dependencies]
affectively-audio-processor = "1.0"
```

## Quick Start

```ts
import init, {
  mix_audio_streams,
  reduce_volume,
  create_silence,
  AudioMixerConfig,
} from '@affectively/wasm-audio-processor';

await init();

const config = new AudioMixerConfig(0.3, 1.0, 100, 100, 8000);
const mixedAudio = mix_audio_streams(originalBase64, whisperBase64, config);
const quieterAudio = reduce_volume(audioBase64, 0.5);
const silence = create_silence(500, 8000);
```

## Why This README Is Grounded

This package does not promise a full audio platform. The strongest fair brag is that it already gives you a useful Rust/WASM utility layer for a handful of common audio operations.
