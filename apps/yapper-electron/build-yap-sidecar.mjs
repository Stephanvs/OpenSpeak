import { chmodSync, copyFileSync, existsSync, mkdirSync, rmSync } from 'node:fs'
import { execFileSync } from 'node:child_process'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const appDir = dirname(fileURLToPath(import.meta.url))
const repoRoot = resolve(appDir, '..', '..')
const yapProjectDir = join(repoRoot, 'services', 'yap')
const yapProjectFile = join(yapProjectDir, 'yap.csproj')
const binaryName = process.platform === 'win32' ? 'yap.exe' : 'yap'
const runtimeIdentifier = getRuntimeIdentifier(process.platform, process.arch)
const publishDir = join(appDir, '.sidecar-build', runtimeIdentifier)
const outputDir = join(appDir, 'resources', 'yap')
const publishedBinary = join(publishDir, binaryName)
const outputBinary = join(outputDir, binaryName)

if (!runtimeIdentifier) {
  throw new Error(`Unsupported platform/architecture: ${process.platform}/${process.arch}`)
}

rmSync(publishDir, { recursive: true, force: true })
rmSync(outputDir, { recursive: true, force: true })
mkdirSync(publishDir, { recursive: true })
mkdirSync(outputDir, { recursive: true })

console.log(`Publishing yap for ${runtimeIdentifier}...`)

execFileSync(
  'dotnet',
  ['publish', yapProjectFile, '-c', 'Release', '-r', runtimeIdentifier, '-o', publishDir],
  {
    cwd: yapProjectDir,
    stdio: 'inherit'
  }
)

if (!existsSync(publishedBinary)) {
  throw new Error(`Expected published yap binary at ${publishedBinary}`)
}

copyFileSync(publishedBinary, outputBinary)

if (process.platform !== 'win32') {
  chmodSync(outputBinary, 0o755)
}

console.log(`Copied yap sidecar to ${outputBinary}`)

function getRuntimeIdentifier(platform, arch) {
  if (platform === 'darwin') {
    if (arch === 'arm64') {
      return 'osx-arm64'
    }

    if (arch === 'x64') {
      return 'osx-x64'
    }
  }

  if (platform === 'win32') {
    if (arch === 'arm64') {
      return 'win-arm64'
    }

    if (arch === 'x64') {
      return 'win-x64'
    }
  }

  if (platform === 'linux') {
    if (arch === 'arm64') {
      return 'linux-arm64'
    }

    if (arch === 'x64') {
      return 'linux-x64'
    }
  }

  return null
}
