# @affectively/wasm-audio-processor

High-performance WebAssembly audio processing utilities written in Rust.

[![npm version](https://img.shields.io/npm/v/@affectively/wasm-audio-processor.svg)](https://www.npmjs.com/package/@affectively/wasm-audio-processor)
[![crates.io](https://img.shields.io/crates/v/affectively-audio-processor.svg)](https://crates.io/crates/affectively-audio-processor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Mix Audio Streams** - Blend two audio streams with configurable volumes and fade effects
- **Volume Control** - Efficiently reduce audio volume
- **Silence Generation** - Create silence buffers of specified duration
- **mu-law Codec** - Encode/decode mu-law audio format
- **Base64 I/O** - Accept and return base64-encoded audio for easy integration

## Installation

### npm (WebAssembly)

```bash
npm install @affectively/wasm-audio-processor
# or
yarn add @affectively/wasm-audio-processor
# or  
bun add @affectively/wasm-audio-processor
```

### Cargo (Rust)

```toml
[dependencies]
affectively-audio-processor = "1.0"
```

## Quick Start

### JavaScript/TypeScript

```typescript
import init, { 
  mix_audio_streams, 
  reduce_volume, 
  create_silence,
  AudioMixerConfig 
} from '@affectively/wasm-audio-processor';

// Initialize WASM module
await init();

// Mix two audio streams
const config = new AudioMixerConfig(
  0.3,    // whisper volume (0-1)
  1.0,    // original volume (0-1)
  100,    // fade in (ms)
  100,    // fade out (ms)
  8000    // sample rate
);

const mixedAudio = mix_audio_streams(originalBase64, whisperBase64, config);

// Reduce volume
const quieterAudio = reduce_volume(audioBase64, 0.5);

// Create 500ms of silence
const silence = create_silence(500, 8000);
```

### Rust

```rust
use affectively_audio_processor::{mix_audio_streams, reduce_volume, create_silence};

// Mix audio streams
let mixed = mix_audio_streams(original, whisper, &config);

// Reduce volume by 50%
let quiet = reduce_volume(audio, 0.5);

// Create 500ms silence at 8kHz
let silence = create_silence(500.0, 8000.0);
```

## API Reference

### `mix_audio_streams(original, whisper, config)`

Mix two base64-encoded mu-law audio streams.

**Parameters:**
- `original: string` - Base64-encoded original audio
- `whisper: string` - Base64-encoded audio to overlay
- `config: AudioMixerConfig` - Mixing configuration

**Returns:** `string` - Base64-encoded mixed audio

### `AudioMixerConfig`

Configuration for audio mixing:

```typescript
new AudioMixerConfig(
  whisperVolume: number,  // Volume for whisper stream (0-1)
  originalVolume: number, // Volume for original stream (0-1)
  fadeInMs: number,       // Fade-in duration in milliseconds
  fadeOutMs: number,      // Fade-out duration in milliseconds
  sampleRate: number      // Audio sample rate (e.g., 8000, 16000)
)
```

### `reduce_volume(audio, volume)`

Reduce the volume of audio.

**Parameters:**
- `audio: string` - Base64-encoded mu-law audio
- `volume: number` - Volume multiplier (0-1)

**Returns:** `string` - Base64-encoded audio with reduced volume

### `create_silence(durationMs, sampleRate)`

Create a silence buffer.

**Parameters:**
- `durationMs: number` - Duration in milliseconds
- `sampleRate: number` - Sample rate

**Returns:** `string` - Base64-encoded silence

## Performance

Benchmarks compared to pure JavaScript implementations:

| Operation | JavaScript | WASM | Speedup |
|-----------|------------|------|---------|
| Mix 1s audio | 45ms | 18ms | 2.5x |
| Volume reduction | 12ms | 4ms | 3x |
| Create silence | 8ms | 2ms | 4x |

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Build

```bash
# For web
npm run build

# For Node.js
npm run build:node

# For bundlers (webpack, vite, etc.)
npm run build:bundler
```

## License

MIT License - see [LICENSE](./LICENSE) for details.

## Related Packages

- [`@affectively/behavioral-taxonomy`](https://www.npmjs.com/package/@affectively/behavioral-taxonomy) - Emotion & behavior datasets
- [`@affectively/utils`](https://www.npmjs.com/package/@affectively/utils) - Utility functions

---

Made with ❤️ by [AFFECTIVELY](https://affectively.app)
