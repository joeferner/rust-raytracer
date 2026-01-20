#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${SCRIPT_DIR}/.."
WEBAPP_DIR="${SCRIPT_DIR}/../../../webapp"

# Parse build mode argument
BUILD_MODE="${1:-all}"

if [[ ! "$BUILD_MODE" =~ ^(debug|release|all)$ ]]; then
    echo "Error: Invalid argument. Use 'debug', 'release', or omit for both."
    echo "Usage: $0 [debug|release]"
    exit 1
fi

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
    
    # Only remove the directories we're about to rebuild
    if [[ "$BUILD_MODE" == "all" ]]; then
        rm -rf "${WEBAPP_DIR}/frontend/src/wasm"
    fi
    
    if [[ "$BUILD_MODE" == "release" || "$BUILD_MODE" == "all" ]]; then
        echo ""
        echo "building release..."
        rm -rf "${WEBAPP_DIR}/frontend/src/wasm/release"
        rm -rf pkg
        wasm-pack build --target web --release
        mkdir -p "${WEBAPP_DIR}/frontend/src/wasm/release"
        cp pkg/caustic_wasm* "${WEBAPP_DIR}/frontend/src/wasm/release"
    fi

    if [[ "$BUILD_MODE" == "debug" || "$BUILD_MODE" == "all" ]]; then
        echo ""
        echo "building debug..."
        rm -rf "${WEBAPP_DIR}/frontend/src/wasm/debug"
        rm -rf pkg
        wasm-pack build --target web --debug
        mkdir -p "${WEBAPP_DIR}/frontend/src/wasm/debug"
        cp pkg/caustic_wasm* "${WEBAPP_DIR}/frontend/src/wasm/debug"
    fi
)

echo "Build complete!"
