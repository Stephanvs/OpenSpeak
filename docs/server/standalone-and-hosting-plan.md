# Server Standalone + Model Hosting Plan

## Goal
The TTS backend should be reusable by all clients and operable independently as a standalone CLI service.

## Backend language decision
- **Selected now**: Rust.
- **Why**: excellent performance, low memory, easy static binaries for macOS/Linux/Windows, strong fit for long-running audio/model services.

## Alternative stacks considered
1. **C# (.NET 8)**
   - Pros: great tooling, strong cross-platform runtime, productive developer experience.
   - Cons: heavier runtime footprint than Rust for minimal server packaging.
2. **TypeScript (Bun)**
   - Pros: very fast iteration, lightweight developer ergonomics.
   - Cons: weaker ecosystem for low-level/native inference integrations compared to Rust/C#.
3. **Go**
   - Pros: simple deploys and ops, static binaries, strong networking.
   - Cons: fewer direct bindings in some AI inference ecosystems than Rust.

## Immediate implementation (MVP)
- Build `services/tts-server` as a Rust `axum` process.
- Include CLI commands for:
  - `serve`
  - `health`
  - `models`
  - `synthesize`
- Implement model inventory abstraction with a stub list now; move to manifest-driven loading next.

## Next steps for model hosting
1. Add model manifest parsing from `models/manifests/*.yaml`.
2. Add local model cache management and checksum validation.
3. Support multiple backend adapters:
   - `piper` (primary cross-platform)
   - `kokoro` (optional)
   - `avspeech` (macOS fallback)
4. Add concurrency and per-model worker pools.
5. Add request cancellation and chunked streaming.

## Standalone CLI usability targets
- Start server in one command.
- Run synthesize in one command and save file.
- No UI dependency required.

## Remote-ability roadmap
- Keep API stateless and host-address configurable by clients.
- Add auth token support and TLS in a later milestone.
- Add CORS and rate limiting policy before internet exposure.
