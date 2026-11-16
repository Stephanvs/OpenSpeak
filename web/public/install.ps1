# Installs the latest openspeak release.
# Usage: irm https://openspeak.sh/install.ps1 | iex
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

$url = 'https://github.com/Stephanvs/OpenSpeak/releases/latest/download/openspeak-x86_64-pc-windows-msvc.zip'
$dir = if ($env:OPENSPEAK_INSTALL_DIR) { $env:OPENSPEAK_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA 'openspeak\bin' }
$zip = Join-Path ([IO.Path]::GetTempPath()) "openspeak-$([guid]::NewGuid()).zip"

try {
    Write-Host "Downloading $url"
    Invoke-WebRequest -Uri $url -OutFile $zip -UseBasicParsing
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
    Expand-Archive -Path $zip -DestinationPath $dir -Force
}
finally {
    Remove-Item $zip -ErrorAction SilentlyContinue
}

$exe = Join-Path $dir 'openspeak.exe'
Write-Host "Installed $(& $exe --version) to $exe"

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if (-not (($userPath -split ';') -contains $dir)) {
    [Environment]::SetEnvironmentVariable('Path', (@($userPath, $dir) | Where-Object { $_ }) -join ';', 'User')
    $env:Path = "$env:Path;$dir"
    Write-Host "Added $dir to your user PATH. Restart your terminal to pick it up."
}
