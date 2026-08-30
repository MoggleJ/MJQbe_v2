#!/usr/bin/env bash
# Test de charge léger sur l'API MJQbe. Utilise `hey` ou `wrk` si présent,
# sinon un fallback curl séquentiel.
#
#   scripts/loadtest.sh [base_url] [requests] [concurrency]
set -euo pipefail

BASE="${1:-http://localhost:4848}"
N="${2:-500}"
C="${3:-20}"
PATHS=("/health" "/apps?mode=tv" "/categories?mode=desktop")

echo "cible : $BASE   requêtes : $N   concurrence : $C"

for p in "${PATHS[@]}"; do
  url="$BASE$p"
  echo "── $url"
  if command -v hey >/dev/null; then
    hey -n "$N" -c "$C" -q 0 "$url" | grep -E 'Requests/sec|Total:|[0-9]+% in'
  elif command -v wrk >/dev/null; then
    wrk -t2 -c"$C" -d10s "$url"
  else
    start=$(date +%s.%N)
    fail=0
    for _ in $(seq 1 "$N"); do
      curl -fsS -o /dev/null "$url" || fail=$((fail + 1))
    done
    end=$(date +%s.%N)
    dur=$(echo "$end - $start" | bc)
    rps=$(echo "scale=1; $N / $dur" | bc)
    echo "  $N requêtes en ${dur}s  →  ${rps} req/s  (échecs : $fail)"
  fi
done
