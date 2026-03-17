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

- Bun
- .NET SDK 10

## Sidecar setup

`bun run sidecar:build` publishes `services/yap` for the current platform and copies the native binary to `resources/yap/` for Electron packaging.

`bun run dev` runs that sidecar build automatically before starting Electron.

## Run locally

```bash
bun install
bun run dev
```

## Package and distribute

```bash
bun run package
bun run make
```

Use `bun run build:mac`, `bun run build:win`, or `bun run build:linux` from a matching environment when you want platform-specific distributables.

## Releases

- GitHub Actions builds release artifacts from `.github/workflows/yapper-release.yml`.
- Push a tag that matches the app version, like `v1.0.0`, to publish a release.
- Manual runs should target an existing `vX.Y.Z` tag that matches `apps/yapper/package.json`.

## Notes

- Generated audio is written to your system temp directory under `yapper-audio`.
- In development, the app can fall back to `dotnet run --project ../../services/yap` if the sidecar binary is missing.
- Packaging and installers now run through Electron Forge with the Vite plugin.
- The migration plan for moving from Tauri to Electron lives in `MIGRATION_PLAN.md`.
