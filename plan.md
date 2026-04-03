**Plan**

Build a new `Tauri 2 + SvelteKit + shadcn-svelte` desktop app inside the existing repo, keep the current Swift macOS app intact as the reference implementation, and target `Windows 11+` first with a phased delivery:

1. `voice memo + mic-only` first
2. `meeting capture with full-system loopback`
3. `Teams / Slack / Zoom process-targeted capture`
4. `speaker diarization` later

This is the safest way to get Windows shipped without blocking on the hardest research item.

**Source Context**

The current macOS app is tightly coupled to Apple APIs and should be treated as the product spec, not the implementation base.

- Tome app architecture and feature map: `E:\Code\Tome\README.md:45-80`, `E:\Code\Tome\README.md:146-175`
- Current macOS-only dependency and packaging model: `E:\Code\Tome\Tome\Package.swift:7-18`
- Current system audio capture uses `ScreenCaptureKit` and per-app filtering: `E:\Code\Tome\Tome\Sources\Tome\Audio\SystemAudioCapture.swift:24-71`
- Current dual-stream orchestration is in `TranscriptionEngine`: `E:\Code\Tome\Tome\Sources\Tome\Transcription\TranscriptionEngine.swift:59-176`
- Current markdown output format is the product contract and should be preserved: `E:\Code\Tome\Tome\Sources\Tome\Storage\TranscriptLogger.swift:30-100`

Handy is the strongest local reference for the new architecture, not for exact feature parity.

- Handy architecture: `E:\Code\Handy\README.md:60-73`
- Handy Windows-ready Rust stack: `E:\Code\Handy\src-tauri\Cargo.toml:31-108`
- Handy updater and signing config: `E:\Code\Handy\src-tauri\tauri.conf.json:26-74`
- Handy app shell layout: `E:\Code\Handy\src\App.tsx:235-270`
- Handy settings sidebar structure: `E:\Code\Handy\src\components\Sidebar.tsx:34-126`
- Handy settings group styling: `E:\Code\Handy\src\components\ui\SettingsGroup.tsx:15-28`
- Handy row layout pattern: `E:\Code\Handy\src\components\ui\SettingContainer.tsx:122-193`

**Locked Decisions**

- Platform priority: `Windows 11+`
- Delivery strategy: `phased rollout`
- Frontend stack: `SvelteKit`
- Desktop shell: `Tauri 2`
- Design system: `shadcn-svelte`
- Initial supported apps for targeted capture: `Teams`, `Slack`, `Zoom`
- Out of scope initially: `Chrome / Google Meet`, Linux implementation, macOS Tauri migration, diarization in v1

**Recommended Technical Stack**

| Concern | Chosen stack | Why |
|---|---|---|
| Desktop shell | `Tauri 2` | Cross-platform repo shape, Windows packaging/updater/signing story, proven by Handy |
| Frontend | `SvelteKit` in SPA mode | Good fit with Tauri, good DX, works well with shadcn-svelte |
| Design system | `shadcn-svelte` | Open-code components, easy to adapt to Handy-like settings UI |
| Typed frontend/backend API | `specta` + `tauri-specta` | Handy already uses this pattern; avoids hand-written bindings |
| Mic capture | `cpal` | Proven, cross-platform abstraction, already used in Handy |
| Windows system audio | `WASAPI` via `wasapi` crate or direct `windows-rs` | Best control for loopback and process-targeted capture |
| STT runtime | `transcribe-rs` + Parakeet ONNX | Closest fit to Tome’s current model choice and Windows viability |
| VAD | `vad-rs` / Silero-style VAD | Proven path from Handy |
| Resampling | `rubato` | Already proven in Handy |
| Windows app detection | Win32 APIs via `windows-rs` | Needed for Teams / Slack / Zoom PID detection |
| Installer | `NSIS` first | Flexible, updater-friendly, Handy already uses it |
| Updater | `tauri-plugin-updater` | Cross-platform and already proven in Handy |
| Signing | `Azure Trusted Signing` or EV cert | Needed to reduce SmartScreen pain |

**Why Not Use `scap` As The Foundation**

`scap` is useful research, but not the primary capture layer for Tome.

- It is primarily a screen/window capture crate: <https://github.com/CapSoftware/scap>
- It exposes `captures_audio`, but on Windows that path is loopback around the default output device, not Tome-grade app-targeted audio routing.
- It does not solve mic capture.
- It does not solve process-targeted loopback.
- It does not solve Teams / Slack / Zoom detection.
- It does not solve Tome’s dual-stream meeting semantics.

Use it only if later you want optional screen/window enumeration or a shared screen-capture abstraction. Do not build the core Windows audio pipeline around it.

**Repository Shape**

Do not replace the Swift app immediately. Add the new stack alongside it.

```text
E:\Code\Tome
├─ Tome/                          # existing Swift macOS app, unchanged initially
├─ apps/
│  └─ desktop/                    # new Tauri + SvelteKit app
│     ├─ src/                     # Svelte frontend
│     ├─ src-tauri/               # Tauri app crate
│     ├─ package.json
│     ├─ svelte.config.js
│     ├─ vite.config.ts
│     └─ components.json
├─ crates/
│  ├─ tome-domain
│  ├─ tome-settings
│  ├─ tome-output
│  ├─ tome-transcribe
│  ├─ tome-audio-core
│  ├─ tome-audio-windows
│  ├─ tome-platform-windows
│  └─ tome-app-api
├─ Cargo.toml                     # workspace root
└─ package.json                   # Bun workspace root
```

**Crate Responsibilities**

- `tome-domain`
  - `SessionType`
  - `Speaker`
  - `Utterance`
  - `DetectedApp`
  - session state enums
- `tome-settings`
  - persisted config schema
  - defaults
  - migration logic
- `tome-output`
  - port of Tome markdown writer
  - session JSONL or equivalent metadata persistence
  - file naming and frontmatter rules
- `tome-transcribe`
  - model registry
  - download/cache/verify
  - `transcribe-rs` orchestration
  - VAD + resampling pipeline
- `tome-audio-core`
  - shared capture traits and event types
- `tome-audio-windows`
  - mic capture
  - full-system loopback
  - process-targeted loopback
- `tome-platform-windows`
  - foreground window detection
  - PID/exe mapping
  - Explorer integration
  - microphone privacy helpers
- `tome-app-api`
  - Tauri commands
  - Tauri events
  - Specta bindings

**Frontend Shape**

Use `SvelteKit` in SPA mode.

- Tauri SvelteKit guidance: <https://v2.tauri.app/start/frontend/sveltekit/>
- Required setup:
  - `@sveltejs/adapter-static`
  - `frontendDist: "../build"`
  - fallback `index.html`
  - root `+layout.ts` with `export const ssr = false`
- `shadcn-svelte` install flow:
  - <https://www.shadcn-svelte.com/docs/installation/sveltekit>
- `shadcn-svelte` sidebar:
  - <https://www.shadcn-svelte.com/docs/components/sidebar>

Use `bun` as the workspace package manager for consistency.

**Frontend State Model**

Use Svelte stores, not a large external state framework.

Create these stores:

- `settingsStore`
- `sessionStore`
- `audioStore`
- `modelStore`
- `updatesStore`

Use Tauri commands for request/response and Tauri events for streaming updates.

Commands:
- `getSettings`
- `setSetting`
- `pickVaultFolder`
- `listModels`
- `downloadModel`
- `cancelModelDownload`
- `startVoiceMemo`
- `startMeetingCapture`
- `stopSession`
- `getSupportedApps`
- `getDetectedForegroundApp`
- `setTargetAppMode`
- `setShortcut`
- `checkForUpdates`
- `installUpdate`

Events:
- `session-state`
- `partial-transcript`
- `final-transcript`
- `audio-levels`
- `model-download-progress`
- `foreground-app-changed`
- `update-progress`
- `capture-error`

**Windows Feature Scope**

**v1**
- Voice memo
- Meeting capture
- Mic capture
- Local STT
- Tome markdown output
- Teams / Slack / Zoom detection metadata
- Handy-like settings UI
- Tray icon
- Global shortcut
- Model download/update flow

**v1.1**
- Full-system loopback meeting capture
- `You` from mic and `Them` from system loopback

**v1.2**
- Process-targeted capture for:
  - Teams
  - Slack
  - Zoom
- Manual app override
- Target lock at session start

**Deferred**
- Chrome / Meet tab detection
- Full remote multi-speaker diarization
- Linux implementation
- macOS migration to the new stack

**Supported App Detection Plan**

Do not overcomplicate app detection initially.

Use this policy:

1. Detect the foreground window only when the app is idle or preparing to start recording.
2. Resolve `HWND -> PID -> executable name/path`.
3. Match only the initial allowlist.
4. When recording starts, lock the target app for that session.
5. Do not switch target app mid-session automatically.
6. If detection fails, expose a manual override.

Initial allowlist:
- Teams
- Slack
- Zoom

Practical matching rules:
- Teams: match modern and legacy executable names, including path-based fallback
- Slack: match `slack.exe`
- Zoom: match `zoom.exe` and path-based fallback

Do not include browser detection or tab detection in this implementation plan.

**Audio Capture Strategy**

**Mic**
- Use `cpal`
- Follow Handy’s overall architecture:
  - device enumeration
  - default device selection
  - stream callbacks
  - resample to `16 kHz mono`
  - VAD before STT
- Relevant Handy references:
  - `E:\Code\Handy\src-tauri\src\audio_toolkit\audio\recorder.rs:65-210`
  - `E:\Code\Handy\src-tauri\src\audio_toolkit\audio\recorder.rs:395-519`
  - `E:\Code\Handy\src-tauri\src\audio_toolkit\audio\resampler.rs:15-99`

**System audio**
Use a two-step rollout.

Step 1:
- implement full-system loopback via WASAPI
- useful for basic meeting capture
- lower research risk

Step 2:
- implement process-targeted loopback for Teams / Slack / Zoom
- use Windows application loopback APIs
- preferred on Windows 11+
- key Microsoft sample:
  - <https://learn.microsoft.com/en-us/samples/microsoft/windows-classic-samples/applicationloopbackaudio-sample/>
- general WASAPI loopback:
  - <https://learn.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording>
- Rust `wasapi` crate:
  - <https://github.com/HEnquist/wasapi-rs>

**Important policy**
- `full-system loopback` is acceptable for early MVP
- `process loopback` is required for the intended Teams / Slack / Zoom experience
- do not block all Windows progress on perfect process loopback from day one

**STT Runtime Strategy**

Use `transcribe-rs` as the main abstraction.

- Handy already uses it:
  - `E:\Code\Handy\src-tauri\Cargo.toml:72,90-108`
- Project:
  - <https://github.com/cjpais/transcribe-rs>

Implementation choice:
- baseline model: Parakeet ONNX int8
- fallback path: Whisper
- VAD: Silero-style path similar to Handy
- CPU baseline first
- DirectML only after CPU baseline is stable and benchmarked

Why:
- closest fit to Tome’s current product behavior
- works in Rust
- Windows-friendly
- avoids depending on `FluidAudio`, which is Apple-platform centered:
  - <https://github.com/FluidInference/FluidAudio>

Do not plan Windows around FluidAudio.

**Transcript And Output Port**

This is one of the first things to port because it is highly reusable and product-defining.

Port directly from Tome:
- session types
- meeting vs voice memo note semantics
- file naming
- YAML frontmatter
- transcript section structure
- context section
- attendee/speaker labeling rules

Key source:
- `E:\Code\Tome\Tome\Sources\Tome\Storage\TranscriptLogger.swift:30-100`
- `E:\Code\Tome\Tome\Sources\Tome\Storage\TranscriptLogger.swift:103-170`
- `E:\Code\Tome\Tome\Sources\Tome\Storage\TranscriptLogger.swift:198-220`

Preserve these frontmatter fields:
- `type`
- `created`
- `time`
- `duration`
- `source_app`
- `source_file`
- `attendees`
- `context`
- `tags`

For Windows v1:
- preserve Tome markdown format exactly where possible
- keep output contract stable so downstream Obsidian/agent workflows do not need to change

**UI Design Plan**

The initial UI should borrow Handy’s structure, not its dictation-specific feature set.

Layout to mimic:
- fixed left sidebar
- scrollable settings content
- grouped cards with subtle borders
- each setting row as `label + tooltip + control`
- simple footer / version area
- compact desktop utility-app feel

Use these Handy files as layout references:
- main shell: `E:\Code\Handy\src\App.tsx:235-270`
- sidebar config: `E:\Code\Handy\src\components\Sidebar.tsx:34-126`
- section group wrapper: `E:\Code\Handy\src\components\ui\SettingsGroup.tsx:15-28`
- row container: `E:\Code\Handy\src\components\ui\SettingContainer.tsx:122-193`

Use these `shadcn-svelte` components:
- `Sidebar`
- `Card`
- `Separator`
- `ScrollArea`
- `Button`
- `Input`
- `Textarea`
- `Select`
- `Switch`
- `Tooltip`
- `Progress`
- `Dialog`
- `Sonner`
- `Kbd`

**Initial Settings IA**

Keep only what matters for Tome.

Sections:
- `General`
- `Models`
- `Advanced`
- `About`

General:
- recording shortcut
- session mode defaults
- microphone selection
- vault folder for meetings
- vault folder for voice memos
- supported app targeting mode
- currently detected app status
- silence auto-stop

Models:
- active model
- installed models
- download progress
- model location
- CPU / DirectML toggle only if needed later

Advanced:
- start hidden
- launch at login
- tray icon
- check for updates
- log folder
- temporary audio retention policy
- manual app override mode for Teams / Slack / Zoom

About:
- app version
- update status
- release notes link
- licenses / acknowledgments

Do not include these Handy-style sections in the first pass:
- history browser
- paste method
- clipboard handling
- translation
- output device selector for playback
- post-processing
- debug section in normal UI

**Main Window Plan**

Keep the main window simple.

It should contain:
- app title / mode badge
- current target app pill
- record / stop CTA
- waveform or audio level indicator
- session status
- optional short live transcript preview
- settings button

Behavior:
- if target app detection is `Auto`, show `Teams`, `Slack`, `Zoom`, or `None`
- if manual target is set, show it clearly
- once session starts, target app label becomes locked for that session

**Onboarding Plan**

First-run onboarding should be minimal and Windows-specific.

Steps:
1. welcome / privacy statement
2. choose meeting transcript folder
3. choose voice memo folder
4. microphone privacy guidance
5. download default model
6. configure recording shortcut
7. explain supported apps:
   - Teams
   - Slack
   - Zoom
   - browser meetings not yet supported

**Detailed Implementation Sequence**

**Phase 0: Bootstrap**
1. Create `apps/desktop` as a new Tauri 2 + SvelteKit project.
2. Configure SvelteKit for Tauri SPA mode.
3. Initialize `shadcn-svelte`.
4. Add a root Rust workspace and keep `Tome/` untouched.
5. Add `specta` and `tauri-specta` so the Svelte frontend gets generated typed bindings.
6. Add Tauri plugins:
   - `dialog`
   - `fs`
   - `opener`
   - `store`
   - `process`
   - `updater`
   - `global-shortcut`
   - `os`
   - `single-instance`
   - `autostart` later if needed
7. Set up Tauri capabilities explicitly instead of broad defaults.
8. Get a blank app booting on Windows with sidebar/settings shell only.

Exit criteria:
- desktop app starts
- Svelte UI renders
- Rust commands call successfully from frontend
- generated bindings are working
- no feature work yet

**Phase 1: Port the product contract**
1. Port Tome domain models into `tome-domain`.
2. Port Tome markdown writer into `tome-output`.
3. Port session persistence semantics.
4. Write golden tests that compare generated markdown to expected Tome output.
5. Define settings schema for vault paths, supported-app behavior, shortcuts, updater, tray, and capture defaults.

Exit criteria:
- can generate a correct Tome-style meeting note from a fixture transcript
- can generate a correct Tome-style voice memo note from a fixture transcript

**Phase 2: Mic-only voice memo**
1. Implement mic device enumeration and selection via `cpal`.
2. Implement resampling to `16 kHz mono`.
3. Implement VAD.
4. Implement `transcribe-rs` model loading, download, caching, and inference.
5. Connect final transcript output to Tome markdown writing.
6. Build the `General`, `Models`, `Advanced`, `About` settings pages.
7. Add global shortcut.
8. Add tray icon and start/stop behavior.

Exit criteria:
- user can record a voice memo on Windows
- model downloads and loads locally
- transcript is written into chosen vault folder in Tome format
- settings UI is usable and Handy-like

**Phase 3: Meeting capture MVP with full-system loopback**
1. Add full-system loopback capture.
2. Run dual-source pipeline:
   - mic -> `You`
   - loopback -> `Them`
3. Merge timeline into transcript store.
4. Ensure session start/stop is robust.
5. Add per-session source app metadata using detection only, not process-targeted capture yet.

Exit criteria:
- can capture a meeting with mic + system audio
- can write a meeting note with `source_app`
- usable for early internal testing

**Phase 4: Teams / Slack / Zoom detection and process-targeted capture**
1. Implement foreground window detection.
2. Implement PID and executable resolution.
3. Build allowlist matcher for:
   - Teams
   - Slack
   - Zoom
4. Add manual override setting.
5. Lock target app at recording start.
6. Implement process-targeted loopback capture for the detected PID tree.
7. Add compatibility handling if process capture returns silence or fails.
8. Build a manual test matrix for Teams / Slack / Zoom.

Exit criteria:
- auto-detection works for supported apps
- target app is stable for the session
- audio capture is limited to that app in normal supported cases
- manual fallback exists when auto-detect is wrong

**Phase 5: Packaging, updater, signing**
1. Configure Tauri bundling with `NSIS` first.
2. Use `embedBootstrapper` or default WebView2 policy based on support preference.
3. Add updater artifacts.
4. Stand up update JSON publishing.
5. Add Azure Trusted Signing or EV signing path.
6. Add CI on `windows-latest`.
7. Publish signed installer + updater assets.

Exit criteria:
- signed installer builds in CI
- updater can check, download, install, relaunch
- install/uninstall path works cleanly on Windows 11

**Phase 6: Hardening**
1. Add transcript golden tests.
2. Add settings migration tests.
3. Add audio pipeline unit tests where possible.
4. Add Playwright smoke tests for settings shell.
5. Add manual hardware matrix testing.
6. Add error handling for:
   - missing mic
   - denied microphone access
   - model download interruption
   - invalid vault path
   - shortcut conflict
   - unsupported target app

Exit criteria:
- app is stable enough for external Windows beta

**Research Spikes That Must Happen Before Real Implementation Starts**

1. `Process loopback spike`
- Validate Teams, Slack, and Zoom specifically.
- Confirm child-process behavior.
- Confirm behavior when user alt-tabs after session start.
- Confirm if process-tree include mode is sufficient.

2. `Model benchmark spike`
- Benchmark Parakeet ONNX int8 with `transcribe-rs` on CPU.
- Benchmark DirectML only after CPU baseline.
- Measure startup latency, RAM, and real-time factor.

3. `Transcript parity spike`
- Ensure the Rust port produces materially identical markdown to current Tome.
- Validate downstream Obsidian/agent workflows against new output.

4. `Shortcut and tray spike`
- Validate global shortcut registration, suspend/resume behavior, and tray interactions on Windows 11.

5. `Installer/update spike`
- Decide final NSIS settings, signing flow, and updater hosting location.
- Confirm SmartScreen experience with chosen certificate strategy.

**Testing Matrix**

Manual app matrix:
- Teams call
- Slack huddle
- Zoom meeting

Case matrix:
- app detected automatically
- app selected manually
- session starts while target app is foreground
- user alt-tabs away after start
- target app closes mid-session
- mic device changes
- no microphone permission
- model missing on first launch
- updater available during idle state

Golden output tests:
- voice memo markdown
- meeting markdown
- tags and frontmatter
- filename format
- source app field
- duration formatting

**Packaging Decisions**

Choose these defaults for the first implementation:

- Installer: `NSIS`
- Install mode: `perUser` initially
- Updater: `tauri-plugin-updater`
- Signing: `Azure Trusted Signing` if available, `EV` if budget allows
- WebView2: `embedBootstrapper` is a good default if you want fewer support issues

Tauri references:
- Windows installer: <https://v2.tauri.app/distribute/windows-installer/>
- Windows signing: <https://v2.tauri.app/distribute/sign/windows/>
- Updater: <https://v2.tauri.app/plugin/updater/>
- Global shortcut: <https://v2.tauri.app/plugin/global-shortcut/>

**Resources**

**Local repo references**
- Tome repo root: `E:\Code\Tome`
- Handy repo root: `E:\Code\Handy`
- Tome product/architecture: `E:\Code\Tome\README.md`
- Tome output contract: `E:\Code\Tome\Tome\Sources\Tome\Storage\TranscriptLogger.swift`
- Tome macOS capture implementation: `E:\Code\Tome\Tome\Sources\Tome\Audio\SystemAudioCapture.swift`
- Handy backend/runtime stack: `E:\Code\Handy\src-tauri\Cargo.toml`
- Handy frontend shell: `E:\Code\Handy\src\App.tsx`
- Handy settings nav: `E:\Code\Handy\src\components\Sidebar.tsx`

**External references**
- Tauri SvelteKit: <https://v2.tauri.app/start/frontend/sveltekit/>
- shadcn-svelte install: <https://www.shadcn-svelte.com/docs/installation/sveltekit>
- shadcn-svelte sidebar: <https://www.shadcn-svelte.com/docs/components/sidebar>
- `scap`: <https://github.com/CapSoftware/scap>
- `wasapi-rs`: <https://github.com/HEnquist/wasapi-rs>
- Windows loopback recording: <https://learn.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording>
- Windows application loopback sample: <https://learn.microsoft.com/en-us/samples/microsoft/windows-classic-samples/applicationloopbackaudio-sample/>
- `transcribe-rs`: <https://github.com/cjpais/transcribe-rs>
- `whisper.cpp`: <https://github.com/ggml-org/whisper.cpp>
- ONNX Runtime DirectML: <https://onnxruntime.ai/docs/execution-providers/DirectML-ExecutionProvider.html>
- FluidAudio: <https://github.com/FluidInference/FluidAudio>

**Important Guidance For The Builder**

- Treat the current Swift app as the behavior reference.
- Treat Handy as the architecture reference.
- Do not attempt to port Swift UI code.
- Do not block the repo setup on Linux or macOS implementation.
- Do not add Chrome / Meet detection into the first build.
- Do not add diarization into the first build unless the spike proves it is low-risk.
- Preserve Tome’s markdown output contract early, because that is the real product surface.

**Recommended First Build Order**

1. bootstrap Tauri + SvelteKit + shadcn-svelte shell
2. port Tome output logic
3. ship mic-only voice memo
4. add full-system loopback meetings
5. add Teams / Slack / Zoom targeted capture
6. add updater/signing
7. revisit diarization
