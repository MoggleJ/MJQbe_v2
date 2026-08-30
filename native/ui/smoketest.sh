#!/usr/bin/env bash
# Build the native UI in a Qt6 container and check the QML tree loads (offscreen).
# Optionally connects to a running mjqbe-core socket (arg 1).
#
#   native/ui/smoketest.sh [/path/to/core.sock]
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOCK="${1:-}"
OUT="$ROOT/.smoke-out"
IMG="mjqbe-native-smoke:latest"

rm -rf "$OUT"; mkdir -p "$OUT"
docker build -f "$ROOT/native/ui/Dockerfile.smoketest" -t "$IMG" "$ROOT" >/dev/null

RUN_ARGS=(-d -v "$OUT":/out)
if [[ -n "$SOCK" && -S "$SOCK" ]]; then
  RUN_ARGS+=(-v "$SOCK":/run/mjqbe/native.sock -e MJQBE_NATIVE_SOCKET=/run/mjqbe/native.sock)
fi

CID=$(docker run "${RUN_ARGS[@]}" "$IMG" sh -c '
  /build/mjqbe-native --windowed > /out/ui.log 2>&1 &
  P=$!; sleep 5
  if kill -0 $P 2>/dev/null; then echo "VERDICT: OK — QML tree loaded" >> /out/ui.log; kill $P
  else echo "VERDICT: FAIL — exited $?" >> /out/ui.log; fi')
sleep 8
docker rm -f "$CID" >/dev/null 2>&1 || true

echo "----- ui.log -----"; cat "$OUT/ui.log"
grep -q "VERDICT: OK" "$OUT/ui.log"
