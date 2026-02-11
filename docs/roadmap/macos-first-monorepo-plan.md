# macOS-First Implementation Plan (Monorepo-Ready)

## Objective
Build a native macOS menubar app for local, offline text-to-speech (TTS), while structuring the repository so Linux and Windows clients can be added later without re-architecture.

## Scope (Current)
- In scope: explicit client/server architecture, Rust backend service + standalone CLI, and macOS menubar frontend scaffold.
- Out of scope: production model runtime and full installer/signing pipeline.

## Product Principles
1. **Offline-first**: core synthesis must work without internet.
2. **Client/server decoupling**: frontend never embeds synthesis logic.
3. **Multi-model hosting**: server can expose multiple AI models concurrently.
4. **Standalone backend usability**: backend runs with or without any desktop UI.
5. **Remote-capable protocol**: same contract works local loopback, LAN, or internet.

## High-Level Architecture

### Core Idea
Implement a long-running **TTS server process** and make every app (macOS now, Windows/Linux later) a thin client over a stable contract.

### Components
- `apps/macos-menubar/`: native SwiftUI menubar client
- `services/tts-server/`: Rust background TTS server and CLI entrypoint
- `packages/contracts/`: API contract source of truth (OpenAPI)
- `packages/sdk/`: future client SDKs per platform

### Why this model
- Enables independent deploy/upgrade of frontend and backend.
- Makes server reusable by CLI automations and non-desktop consumers.
- Prevents cross-platform drift by enforcing one transport contract.

---

## API & Runtime Strategy

## Contract v1 (MVP)
- `GET /v1/health`
- `GET /v1/models`
- `POST /v1/synthesize`
- `GET /v1/audio/{request_id}`

## Server runtime modes
1. **Embedded local mode**: started in background by macOS app.
2. **Standalone mode**: started directly via CLI.
3. **Remote mode**: same API hosted on another machine.

## Model hosting strategy
- Initial: in-memory stub model inventory.
- Next: manifest-driven model loading and backend adapters.
- Target: multiple active model backends with per-model resource governance.

---

## Implementation Phases

## Phase 0 — Foundation (completed)
- Repo layout and architecture docs.
- macOS-first roadmap and initial backlog.

## Phase 1 — Client/Server baseline (in progress)
- Define OpenAPI contract v1.
- Scaffold Rust backend service with contract endpoints.
- Scaffold standalone backend CLI for `serve`, `health`, `models`, `synthesize`.
- Scaffold macOS menubar app folder with API client stubs.

**Exit criteria**
- CLI can run server and synthesize output file.
- macOS client scaffold can resolve model list endpoint.

## Phase 2 — Real synthesis adapters
- Add manifest-driven model loader.
- Add first production inference adapter (e.g., Piper).
- Keep macOS fallback adapter path available.

## Phase 3 — Menubar UX + playback
- Implement full menubar interactions.
- Add audio playback and robust error messaging.
- Add endpoint configuration (local/remote).

## Phase 4 — Packaging and hardening
- background process management lifecycle on macOS.
- signing/notarization approach.
- performance and resilience pass.

## Phase 5 — Cross-platform enablement
- Stand up Windows and Linux app clients using same contract.
- add shared contract compatibility CI.

---

## Governance
- Any contract-breaking change requires new version and migration notes.
- App/frontend work must reference contract version in docs/progress logs.
- Every milestone updates `docs/progress/` with implementation status.
