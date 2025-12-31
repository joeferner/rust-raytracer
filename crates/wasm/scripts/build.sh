#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/.."
WEBAPP_DIR="${SCRIPT_DIR}/../../../webapp"

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
    mkdir -p "${WEBAPP_DIR}/frontend/src/wasm"
    cp pkg/rust_raytracer_wasm* "${WEBAPP_DIR}/frontend/src/wasm"
)

echo "Build complete!"
