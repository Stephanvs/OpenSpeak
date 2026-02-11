# Yapper macOS Menubar App (Scaffold)

This is a native SwiftUI menubar app scaffold that talks to the decoupled TTS server.

## Notes
- This folder is scaffolded in this Linux environment and must be opened on macOS with Xcode to build/run.
- Architecture target is client/server only; no synthesis logic is embedded in app.

## Planned behavior
1. Menubar icon opens popover.
2. Popover contains text input + model picker + Synthesize button.
3. App invokes `POST /v1/synthesize`, then fetches `/v1/audio/{request_id}`.
4. Audio is played locally via AVFoundation.

## Local endpoint
- Default API endpoint: `http://127.0.0.1:5100`.
- Should be configurable later via settings.
