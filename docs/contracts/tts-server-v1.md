# TTS Server Contract v1

This contract defines the decoupled client/server boundary so frontend apps (macOS now, Windows/Linux later) can integrate without embedding synthesis code.

## Transport
- HTTP/JSON over local loopback by default (`127.0.0.1:5100`).
- Can be deployed remotely over LAN/Internet later using the same API surface.

## Endpoints
1. `GET /v1/health` — readiness and model count.
2. `GET /v1/models` — lists all hosted model IDs and metadata.
3. `POST /v1/synthesize` — creates synthesis request and returns a stream URL.
4. `GET /v1/audio/{request_id}` — fetches resulting audio payload (WAV for MVP).

## Client Workflow
1. Client checks `/v1/health` at startup.
2. Client loads voices using `/v1/models`.
3. Client submits text to `/v1/synthesize` with selected `model_id`.
4. Client streams/downloads audio from `/v1/audio/{request_id}`.

## Multi-model hosting requirement
- Server must support many models simultaneously.
- Every synthesis request must be explicit about `model_id`.
- A single UI should be able to switch models per request.

## Deployment modes
- **Embedded local mode**: server started as background process by app.
- **Standalone mode**: server run manually (CLI), app points to configured endpoint.
- **Remote mode**: same client contract, server on LAN/Internet with auth/TLS to be added in later phase.

## Reference schema
- Canonical OpenAPI file: `packages/contracts/openapi/tts-server-v1.yaml`.
