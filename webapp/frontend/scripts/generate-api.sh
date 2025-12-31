#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "${SCRIPT_DIR}/.."

if [ -f /app/openapi.json ]; then
    echo "using /app/openapi.json"
    cp /app/openapi.json /tmp/openapi.json
else
    echo "generating new openapi.json"
    "${SCRIPT_DIR}/../../../target/debug/rust-raytracer-webapp" --write-swagger /tmp/openapi.json
fi

npx openapi-typescript-codegen \
  --input /tmp/openapi.json \
  --output ./src/api \
  --client fetch \
  --name RayTracerApi

echo "complete!"
