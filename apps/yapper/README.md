# Yapper Electron

Electron + Svelte desktop launcher for text-to-speech.

## What it does

- Registers a global shortcut: `Cmd/Ctrl + Shift + Y`
- Shows a small popup window with a textarea
- Adds a tray icon with a native menu (`Toggle UI`, `Quit`)
- Uses `services/yap` to generate audio files in the system temp directory

## Starter pack

This app was scaffolded from the `@quick-start/electron` starter using the `svelte-ts` template.

## Requirements

- Node.js + npm
- .NET SDK 10

## Sidecar setup

`npm run sidecar:build` publishes `services/yap` for the current platform and copies the native binary to `resources/yap/` for Electron packaging.

`npm run dev` runs that sidecar build automatically before starting Electron.

## Run locally

```bash
npm install
npm run dev
```

## Build

```bash
npm run build:mac
```

You can also use `npm run build:win` or `npm run build:linux` from a matching environment.

## Notes

- Generated audio is written to your system temp directory under `yapper-audio`.
- In development, the app can fall back to `dotnet run --project ../../services/yap` if the sidecar binary is missing.
- The migration plan for moving from Tauri to Electron lives in `MIGRATION_PLAN.md`.
