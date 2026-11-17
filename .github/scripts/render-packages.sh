#!/usr/bin/env bash
# Usage: render-packages.sh <version> <dist-dir> <out-dir>
# Reads openspeak-<target>.{tar.gz,zip} from <dist-dir> and writes the
# Homebrew formula and Scoop manifest for that release into <out-dir>.
set -euo pipefail

version="$1"
dist="$2"
out="$3"
repo="https://github.com/Stephanvs/OpenSpeak"
base="$repo/releases/download/v$version"

sha() {
  local file="$dist/openspeak-$1"
  [[ -f "$file" ]] || { echo "missing release asset $file" >&2; exit 1; }
  sha256sum "$file" | cut -d' ' -f1
}

mac_arm=$(sha aarch64-apple-darwin.tar.gz)
mac_x64=$(sha x86_64-apple-darwin.tar.gz)
linux_arm=$(sha aarch64-unknown-linux-gnu.tar.gz)
linux_x64=$(sha x86_64-unknown-linux-gnu.tar.gz)
win_x64=$(sha x86_64-pc-windows-msvc.zip)

mkdir -p "$out"

cat > "$out/openspeak.rb" <<EOF
class Openspeak < Formula
  desc "Small command-line text-to-speech client"
  homepage "https://openspeak.sh"
  version "$version"
  license "MIT"

  on_macos do
    on_arm do
      url "$base/openspeak-aarch64-apple-darwin.tar.gz"
      sha256 "$mac_arm"
    end
    on_intel do
      url "$base/openspeak-x86_64-apple-darwin.tar.gz"
      sha256 "$mac_x64"
    end
  end

  on_linux do
    on_arm do
      url "$base/openspeak-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "$linux_arm"
    end
    on_intel do
      url "$base/openspeak-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "$linux_x64"
    end
  end

  def install
    bin.install "openspeak"
  end

  def caveats
    on_linux do
      <<~CAVEATS
        openspeak loads your distribution's ALSA library at runtime.
        On Debian and Ubuntu: sudo apt install libasound2t64 (libasound2 before 24.04)
      CAVEATS
    end
  end

  test do
    assert_match "openspeak #{version}", shell_output("#{bin}/openspeak --version")
  end
end
EOF

jq -n \
  --arg version "$version" \
  --arg url "$base/openspeak-x86_64-pc-windows-msvc.zip" \
  --arg hash "$win_x64" \
  --arg repo "$repo" \
  '{
    version: $version,
    description: "Small command-line text-to-speech client",
    homepage: "https://openspeak.sh",
    license: "MIT",
    architecture: { "64bit": { url: $url, hash: $hash } },
    bin: "openspeak.exe",
    checkver: { github: $repo },
    autoupdate: {
      architecture: {
        "64bit": { url: ($repo + "/releases/download/v$version/openspeak-x86_64-pc-windows-msvc.zip") }
      }
    }
  }' > "$out/openspeak.json"
