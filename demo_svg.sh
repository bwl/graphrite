#!/usr/bin/env bash
set -euo pipefail

# Demo: Render SVG from Rube Goldberg example
# Requires: graphviz 'dot'

if ! command -v dot >/dev/null 2>&1; then
  echo "error: graphviz 'dot' not found in PATH" >&2
  exit 1
fi

FILE="samples/valid/rube_goldberg_posting.mmd"
OUT=${1:-/tmp/rube_goldberg.svg}

if [ ! -f "$FILE" ]; then
  echo "error: sample not found: $FILE" >&2
  exit 1
fi

cargo run -q -p graphrite-cli -- render --format dot "$FILE" | dot -Tsvg > "$OUT"
echo "wrote $OUT"
