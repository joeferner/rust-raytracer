#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

cd "${SCRIPT_DIR}/.."

cargo build --workspace --exclude caustic-wasm --release
flamegraph -o flamegraph.svg -- target/release/caustic-cli

echo "complete!"
