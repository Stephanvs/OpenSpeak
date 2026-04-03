# Yapper

Windows-first desktop app built with SvelteKit, Tauri, Bun, and Rust.

## Prerequisites

Yapper currently targets Windows because the native desktop crates are Windows-specific.

Install these dependencies before trying to run or build the app:

1. Bun

```powershell
bun --version
```

The workspace is pinned to `bun@1.3.11` in the root `package.json`.

2. Rust stable with the MSVC toolchain

```powershell
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
rustc --version
cargo --version
```

The desktop crate declares `rust-version = "1.77.2"`, so any recent stable toolchain is fine.

3. Visual Studio Build Tools 2022 or Visual Studio 2022 with the C++ build tools installed

Required pieces:

- MSVC x64/x86 build tools
- Windows 10/11 SDK

The easiest option is to install the `Desktop development with C++` workload.

This repo uses `scripts/run-tauri-with-msvc.ps1` to locate a working MSVC toolchain automatically before invoking Tauri.

4. Microsoft Edge WebView2 Runtime

Tauri uses WebView2 on Windows. It is usually already installed on Windows 11, but if it is missing the desktop app will not start correctly.

## Install dependencies

From the repository root:

```powershell
bun install
```

You do not need to install the Tauri CLI globally. The repo uses the local package from `apps/yapper/node_modules`.

## Running the app

Browser-only frontend dev server from the repo root:

```powershell
bun run desktop:dev
```

Desktop Tauri dev mode from the repo root:

```powershell
bun run --cwd apps/yapper tauri dev
```

## Building the desktop app

From the repo root:

```powershell
bun run desktop:tauri:build
```

The built executable is written to:

```text
target/release/yapper-desktop.exe
```

## Useful checks

```powershell
bun run desktop:check
bun run desktop:lint
bun run desktop:test
```

## Troubleshooting

If the Rust build fails with errors like `vcruntime.h` or `excpt.h` not found, your MSVC environment is incomplete. Install or repair Visual Studio Build Tools 2022 with the C++ workload and Windows SDK.

If `bun run --cwd apps/yapper tauri dev` or `bun run desktop:tauri:build` cannot find a usable Visual Studio install, make sure at least one installation includes the `Microsoft.VisualStudio.Component.VC.Tools.x86.x64` component.
