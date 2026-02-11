# Architecture Overview

Yapper is a monorepo for a decoupled TTS platform.

## Current delivery focus
- First target platform: macOS menubar app.
- Shared Rust backend service architecture designed for Windows/Linux client reuse.

## Architectural pattern
- Clients in `apps/*` (native UX only).
- Shared server in `services/tts-server` (model hosting + synthesis).
- Contract source of truth in `packages/contracts`.

## Boundary rules
1. `apps/*` never host inference runtime.
2. `services/tts-server` owns synthesis behavior and model inventory.
3. `packages/contracts` versioning gates compatibility.
4. Clients may run local or remote server endpoints without codepath changes.

## Runtime modes
- Embedded local server (app-managed background process).
- Standalone server (CLI-managed process).
- Remote server (LAN/Internet), same API contract.

## Evolution path
1. Finalize server v1 contract and baseline CLI.
2. Complete macOS menubar client + playback against contract.
3. Add production model adapters and manifest loading.
4. Add Windows/Linux clients against unchanged core API.
