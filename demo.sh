#!/usr/bin/env bash
set -euo pipefail

# Demo: Graphrite CLI end-to-end
# Usage: ./demo.sh [file]

FILE=${1:-/tmp/graphrite_demo.mmd}

cat > "$FILE" <<'EOF'
%% Diagram: Demo

direction LR

start["Start"]
end["End"]
start --> end
EOF

echo "== check =="
cargo run -q -p graphrite-cli -- check "$FILE"

echo "== parse (AST) =="
cargo run -q -p graphrite-cli -- parse "$FILE" | head -n 12

echo "== lint (pretty) =="
cargo run -q -p graphrite-cli -- lint --pretty "$FILE" || true

echo "== render dot =="
cargo run -q -p graphrite-cli -- render --format dot "$FILE" | tee /tmp/graphrite_demo.dot | sed -n '1,6p'

echo "== render svg (requires graphviz 'dot') =="
if command -v dot >/dev/null 2>&1; then
  cargo run -q -p graphrite-cli -- render --format dot "$FILE" | dot -Tsvg > /tmp/graphrite_demo.svg
  echo "wrote /tmp/graphrite_demo.svg"
else
  echo "dot not found; skipping svg"
fi
