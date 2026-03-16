# Yapper Electron Migration Plan

## Goal

Replace the current Tauri desktop shell with an Electron shell while keeping the existing `services/yap` backend contract intact.

## Starter Choice

- Starter pack: `@quick-start/electron`
- Template: `svelte-ts`
- New app folder: `apps/yapper-electron`

This keeps the migration scope narrow: Electron main/preload replace Tauri, while the Svelte renderer and `yap` sidecar workflow stay conceptually the same.

## Minimal Parity Target

The first Electron milestone should preserve the behavior users already have in `apps/yapper`:

1. A hidden launcher window that can be toggled globally with `Cmd/Ctrl + Shift + Y`
2. A simple tray/menu-bar presence with `Toggle UI` and `Quit`
3. A textarea-based UI for generating audio files
4. Reuse of `services/yap` to synthesize audio into a temp directory
5. A way to reveal the generated output in the OS file manager
6. Packaged builds that include the `yap` binary so end users do not need `dotnet`

## Tauri To Electron Mapping

| Current Tauri area | Electron replacement |
| --- | --- |
| `src-tauri/src/lib.rs` window lifecycle | `src/main/index.ts` |
| Tauri command `synthesize_text` | `ipcMain.handle('yapper:synthesize-text')` |
| `@tauri-apps/api/core.invoke` | `window.api.synthesizeText()` via preload |
| `getCurrentWindow().hide()` | `window.api.hideLauncher()` |
| `plugin-opener` reveal/open path | `window.api.revealOutput()` + Electron shell |
| `src-tauri/build.rs` sidecar publishing | `build-yap-sidecar.mjs` |
| `tauri.conf.json` bundle sidecar config | `electron-builder.yml` `extraResources` |
| SvelteKit SPA route `src/routes/+page.svelte` | renderer root `src/renderer/src/App.svelte` |

## Execution Plan

### Phase 1: Scaffold And Rename

- Create a fresh Electron app beside the Tauri app instead of editing `apps/yapper` in place
- Keep the generated project close to the starter pack so updates and troubleshooting stay easy
- Rename product metadata and packaging identifiers to match Yapper

### Phase 2: Rebuild The Desktop Shell

- Create a hidden `BrowserWindow` sized like the current launcher
- Intercept the close action so the window hides instead of exiting
- Register the global shortcut in the main process
- Add a tray icon and native menu for `Toggle UI` and `Quit`
- Keep the renderer isolated behind a preload bridge instead of exposing Node APIs directly

### Phase 3: Recreate The Sidecar Contract

- Port the current `synthesize_text` flow from Rust to Electron main process code
- Keep the same validation and temp output directory behavior
- Publish `services/yap` for the active platform with a Node build helper
- Package that binary with Electron Builder using `extraResources`
- In development, allow a fallback to `dotnet run` when no published binary is present

### Phase 4: Port The Renderer

- Move the current launcher UI into the Electron renderer with minimal visual change
- Replace Tauri APIs with preload-exposed functions
- Preserve the current keyboard shortcuts in the UI: `Cmd/Ctrl + Enter` and `Esc`
- Keep the UI as a single-purpose launcher rather than redesigning it yet

### Phase 5: Validate Packaging And Runtime

- Verify `npm run dev` starts the app and can synthesize audio end to end
- Verify the packaged app can find the bundled `yap` binary
- Check tray behavior and global shortcut behavior on macOS first
- Confirm output reveal behavior in Finder/File Explorer

### Phase 6: Follow-Up Migration Work

- Decide whether `apps/yapper` should later be renamed, replaced, or archived
- Port any remaining Tauri-only polish not included in the minimal milestone
- Add app icons, signing/notarization, and release automation once parity is stable
- Revisit package-manager consistency if we want this Electron app to use Bun instead of npm

## Risks And Open Items

- Electron tray behavior differs by platform; macOS will likely be the primary target for parity first
- Global shortcuts can fail if the accelerator is already in use
- Electron packaging paths differ from Tauri sidecars, so runtime path resolution must stay explicit
- The current repo still contains the Tauri app; we should avoid renaming or deleting it until Electron parity is verified
- `services/yap/appsettings.json` appears sensitive and should stay out of any packaged or committed secret flow decisions

## Recommended Next Implementation Order

1. Get dev-mode synthesize flow working in `apps/yapper-electron`
2. Run a packaged macOS build and verify the bundled sidecar path
3. Compare Electron behavior against `apps/yapper` for hotkey, tray, hide-on-close, and reveal flow
4. Once stable, decide how to transition or retire the Tauri app
