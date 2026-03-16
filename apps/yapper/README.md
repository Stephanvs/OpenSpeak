# Yapper Desktop

Tauri + SvelteKit desktop launcher for text-to-speech.

## What it does

- Registers a global shortcut: `Cmd/Ctrl + Shift + Y`
- Shows a small popup window with a textarea
- Adds a macOS menubar icon with a native menu (`Toggle UI`, `Quit`)
- Bundles `services/yap` as a native Tauri sidecar to generate audio files

## Requirements

- Bun
- Rust toolchain
- .NET SDK 10

## Sidecar setup

No manual install is needed.
When Tauri builds or runs the desktop app, `src-tauri/build.rs` publishes `services/yap` for the active target and places the generated sidecar in `apps/yapper/src-tauri/binaries/`.

## Run locally

```bash
bun install
bun run tauri dev
```

## Notes

- Generated audio is written to your system temp directory under `yapper-audio`.
- The packaged app invokes the bundled `yap` sidecar, so end users do not need `dotnet` installed.
