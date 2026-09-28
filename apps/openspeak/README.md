# OpenSpeak

OpenSpeak is a small command-line text-to-speech client written in Rust. It can
play generated speech immediately or save it to an audio file.

## Configuration

The Rust application is configured exclusively through environment variables.
It does not load the legacy `appsettings.json` file.

| Variable | Default | Purpose |
| --- | --- | --- |
| `OPENSPEAK_DEFAULT_PROVIDER` | `grok` | Provider used when `--provider` is omitted |
| `XAI_API_KEY` | — | xAI API key |
| `XAI_API_BASE_URL` | `https://api.x.ai` | xAI API base URL |
| `XAI_DEFAULT_VOICE_ID` | `eve` | Default xAI voice or custom voice ID |
| `XAI_DEFAULT_LANGUAGE` | `auto` | Default xAI BCP-47 language code |
| `XAI_DEFAULT_OUTPUT_FORMAT` | `mp3` | xAI codec or `codec_sampleRate_bitRate` token |
| `XAI_DEFAULT_SAMPLE_RATE` | `24000` | Default xAI sample rate |
| `XAI_DEFAULT_BIT_RATE` | `128000` | Default xAI MP3 bit rate |
| `ELEVENLABS_API_KEY` | — | ElevenLabs API key |
| `ELEVENLABS_API_BASE_URL` | `https://api.elevenlabs.io` | ElevenLabs API base URL |
| `ELEVENLABS_DEFAULT_VOICE_ID` | — | Default ElevenLabs voice ID |
| `ELEVENLABS_DEFAULT_MODEL_ID` | `eleven_multilingual_v2` | Default ElevenLabs model ID |
| `ELEVENLABS_DEFAULT_OUTPUT_FORMAT` | `mp3_44100_128` | Default ElevenLabs output format token |

For example:

```bash
export XAI_API_KEY="..."
cargo run --manifest-path apps/openspeak/Cargo.toml -- speak "Good morning."
```

ElevenLabs requires a voice ID either through
`ELEVENLABS_DEFAULT_VOICE_ID` or `--voice`.

## Commands

```bash
openspeak speak "Good morning."
openspeak say "The speak command also has a short alias."
openspeak synthesize "Ship it." --output ./audio
openspeak devices list
openspeak speak "Play this on my headset." --device 2
```

Use `openspeak <command> --help` for provider, voice, model, codec, speed,
sample-rate, bit-rate, and language overrides.

## Audio playback

`devices list` shows available output devices with short numbers, such as
`0: Speakers (default)`. Pass a number to `speak --device` to play on that
device. If `--device` is omitted, speech uses the system default.

The list is sorted by CPAL's stable device identifiers, so numbers stay the
same between runs while the available device set stays the same. Adding or
removing a device can change the numbers; run `devices list` again after a
hardware or audio service change. Friendly names can repeat, but each number
selects its corresponding device. No long device identifier needs to be copied.

CPAL supplies both enumeration and playback device handles through WASAPI on
Windows, Core Audio on macOS, and ALSA on Linux. Rodio decodes and plays the
generated audio. Linux builds need ALSA development headers (`libasound2-dev`
on Debian/Ubuntu). Immediate playback supports MP3 and WAV output. Use
`synthesize` to save other codecs.

## Development

```bash
cargo fmt --manifest-path apps/openspeak/Cargo.toml --check
cargo clippy --manifest-path apps/openspeak/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path apps/openspeak/Cargo.toml
```
