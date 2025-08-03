#!/usr/bin/env bash
set -euo pipefail

# Generate SVGs for all golden valid samples using Graphviz 'dot'
# Usage: ./gen_golden_svgs.sh [outdir]

if ! command -v dot >/dev/null 2>&1; then
  echo "error: graphviz 'dot' not found in PATH" >&2
  exit 1
fi

OUTDIR=${1:-/tmp/graphrite_svgs}
mkdir -p "$OUTDIR"

for f in samples/valid/*.mmd; do
  name=$(basename "$f" .mmd)
  echo "rendering $name.svg"
  cargo run -q -p graphrite-cli -- render --format dot "$f" | dot -Tsvg > "$OUTDIR/$name.svg"
done

echo "done -> $OUTDIR"
