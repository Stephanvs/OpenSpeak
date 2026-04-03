param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$TauriArgs
)

$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$appDir = Join-Path $repoRoot 'apps\yapper'

function Get-TauriCliPath {
    $candidates = @(
        (Join-Path $appDir 'node_modules\.bin\tauri.exe'),
        (Join-Path $appDir 'node_modules\.bin\tauri.cmd'),
        (Join-Path $repoRoot 'node_modules\.bin\tauri.exe'),
        (Join-Path $repoRoot 'node_modules\.bin\tauri.cmd')
    )

    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }

    throw 'Could not find the Tauri CLI in app or workspace node_modules. Run bun install first.'
}

function Import-MsvcEnvironment {
    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
    if (-not (Test-Path $vswhere)) {
        throw 'Could not find vswhere.exe. Install Visual Studio Build Tools 2022 with the C++ toolchain.'
    }

    $installationPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if (-not $installationPath) {
        throw 'Could not find a Visual Studio installation with MSVC build tools. Install Desktop development with C++ or the VC x64/x86 build tools component.'
    }

    $vsDevCmd = Join-Path $installationPath 'Common7\Tools\VsDevCmd.bat'
    if (-not (Test-Path $vsDevCmd)) {
        throw "Could not find VsDevCmd.bat under '$installationPath'."
    }

    $environmentLines = & cmd.exe /d /s /c ('"' + $vsDevCmd + '" -arch=x64 -host_arch=x64 >nul && set')
    foreach ($line in $environmentLines) {
        if ($line -match '^(.*?)=(.*)$') {
            Set-Item -Path ("Env:{0}" -f $matches[1]) -Value $matches[2]
        }
    }
}

if ($env:OS -eq 'Windows_NT') {
    Import-MsvcEnvironment
}

$tauriCli = Get-TauriCliPath

Push-Location $appDir
try {
    & $tauriCli @TauriArgs
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
