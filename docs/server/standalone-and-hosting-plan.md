# Server Standalone + Model Hosting Plan

## Goal
The TTS backend should be reusable by all clients and operable independently as a standalone CLI service.

## Immediate implementation (MVP)
- Build `services/tts-server` as a Python FastAPI process.
- Include CLI commands for:
  - `serve`
  - `health`
  - `models`
  - `synthesize`
- Implement model inventory abstraction with a stub list now; move to manifest-driven loading next.

## Next steps for model hosting
1. Add model manifest parsing from `models/manifests/*.yaml`.
2. Add local model cache management and validation checksums.
3. Support multiple backend adapters:
   - `piper` (primary cross-platform)
   - `avspeech` (macOS fallback)
4. Add concurrency and per-model worker pools.
5. Add request cancellation and audio chunked streaming.

## Standalone CLI usability targets
- Start server in one command.
- Run synthesize in one command and save file.
- No UI dependency required.

## Remote-ability roadmap
- Keep API stateless and host-address configurable by clients.
- Add auth token support and TLS in a later milestone.
- Add CORS and rate limiting policy before internet exposure.
