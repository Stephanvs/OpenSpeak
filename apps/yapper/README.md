# Yapper Desktop

Tauri + SvelteKit desktop launcher for text-to-speech.

## What it does

- Registers a global shortcut: `Cmd/Ctrl + Shift + Y`
- Shows a small popup window with a textarea
- Adds a macOS menubar icon with a native menu (`Toggle UI`, `Quit`)
- Executes the published `yap` native sidecar binary to generate audio files

## Requirements

- Bun
- Rust toolchain

## Sidecar setup

Install the published `yap` native binary and make sure it is available on your `PATH`.
If needed, point the app to a custom location with `YAP_BINARY_PATH=/absolute/path/to/yap`.

## Run locally

```bash
bun install
bun run tauri dev
```

## Notes

- Generated audio is written to your system temp directory under `yapper-audio`.
- The app invokes the `yap` native binary directly, so it has no runtime dependency on `dotnet`.
