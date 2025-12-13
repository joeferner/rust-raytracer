#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

banner() {
    local title="$1"
    
    # Colors
    local CYAN='\033[0;36m'
    local YELLOW='\033[1;33m'
    local NC='\033[0m' # No Color
    
    echo -e ""
    echo -e " ${YELLOW}${title}"
    echo -e "${CYAN}=============================================================${NC}"
}

banner "cargo fmt"
cd "${SCRIPT_DIR}/.."
cargo fmt

banner "cargo clippy"
cargo clippy --workspace --exclude rust-raytracer-wasm -- -Dwarnings
cargo clippy -p rust-raytracer-wasm --target wasm32-unknown-unknown -- -Dwarnings

banner "cargo build"
cargo build --workspace --exclude rust-raytracer-wasm
cargo build -p rust-raytracer-wasm --target wasm32-unknown-unknown

banner "cargo test"
cargo test --workspace --exclude rust-raytracer-wasm

banner "wasm-pack"
cd "${SCRIPT_DIR}/../crates/wasm"
wasm-pack build --target web --release

cd "${SCRIPT_DIR}/../web-app"
banner "npm format"
npm run format

banner "npm lint"
npm run lint

banner "npm build"
npm run build

echo ""
echo "complete!"
