#!/usr/bin/env bash
# Build the whisper-loadable cdylib + host binary, then run the host
# against the freshly-built .so. Optional env:
#
#   WHISPER_MODEL     tiny|base|small|medium|large-v3   (default: tiny)
#   WHISPER_LANGUAGE  ISO-639-1 code or "auto"          (default: en)
#   WHISPER_DEVICE    auto|cpu|cuda|metal               (default: cpu)
#   WHISPER_DEMO_WAV  path to a 16 kHz mono WAV         (optional;
#                                                        falls back to 5s silence)
set -euo pipefail
cd "$(dirname "$0")"

echo "[run] building plugin (cdylib)..."
cargo build

echo "[run] building host..."
( cd host && cargo build )

PLUGIN_BASENAME="libwhisper_loadable_plugin.so"
case "$(uname)" in
  Darwin)  PLUGIN_BASENAME="libwhisper_loadable_plugin.dylib" ;;
  MINGW*|MSYS*|CYGWIN*) PLUGIN_BASENAME="whisper_loadable_plugin.dll" ;;
esac
PLUGIN="$(pwd)/target/debug/${PLUGIN_BASENAME}"

echo "[run] plugin: ${PLUGIN}"
ls -lh "${PLUGIN}"

WHISPER_MODEL="${WHISPER_MODEL:-tiny}" \
WHISPER_LANGUAGE="${WHISPER_LANGUAGE:-en}" \
WHISPER_DEVICE="${WHISPER_DEVICE:-cpu}" \
WHISPER_DEMO_WAV="${WHISPER_DEMO_WAV:-}" \
  ./host/target/debug/whisper-loadable-host "${PLUGIN}"
