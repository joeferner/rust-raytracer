#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/.."

# Check for wasm-pack
if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "wasm-pack not found. Installing..."
    cargo install wasm-pack
    echo "wasm-pack installed successfully!"
else
    echo "wasm-pack already installed."
fi


# Build WASM using wasm-pack
echo "Running wasm-pack build…"
(
    cd "$PROJECT_DIR"
    wasm-pack build --target web --release
)

echo "Build complete!"
