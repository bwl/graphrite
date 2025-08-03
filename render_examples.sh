#!/usr/bin/env bash
set -euo pipefail

# Render all valid samples into docs/examples as SVG using internal renderer
# Usage: ./render_examples.sh

OUTDIR=docs/examples
mkdir -p "$OUTDIR"

for f in samples/valid/*.mmd; do
  name=$(basename "$f" .mmd)
  echo "rendering $name.svg"
  cargo run -q -p graphrite-cli -- render --format svg "$f" > "$OUTDIR/$name.svg"
done

echo "done -> $OUTDIR"
