#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../../services/tts-server"
cargo run -- serve --host 127.0.0.1 --port 5100
