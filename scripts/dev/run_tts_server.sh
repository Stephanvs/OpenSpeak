#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../../services/tts-server"
python -m pip install -e .
yapper-server serve --host 127.0.0.1 --port 5100
