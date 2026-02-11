# macOS Implementation Backlog (Menubar Client)

## Status tracking
- [x] Create macOS menubar app scaffold folder and SwiftUI stubs.
- [x] Define decoupled client/server contract and endpoint flow.
- [ ] Create Xcode project and verify compile/run on macOS host.

## Menubar UX
- [ ] Menubar icon + popover behavior polish.
- [ ] Input textbox with sensible length checks.
- [ ] Model picker sourced from `/v1/models`.
- [ ] Synthesize action from `/v1/synthesize`.
- [ ] Playback from `/v1/audio/{request_id}`.

## Runtime behavior
- [ ] Configurable endpoint (localhost default, remote optional).
- [ ] Embedded background server process lifecycle (start/stop/health watchdog).
- [ ] Retry/backoff behavior when backend unavailable.

## Quality
- [ ] App launch smoke test on macOS.
- [ ] End-to-end synthesize + playback smoke test.
- [ ] Basic accessibility pass (labels, keyboard, VoiceOver hints).
- [ ] Error copy for model-missing/network-unavailable cases.

## Release readiness
- [ ] Developer runbook for backend + app launch.
- [ ] Code signing and notarization checklist.
- [ ] Packaging plan including backend binary + models.
