#!/bin/bash
set -eux

# Set PATH to latest version of`wasm-opt`

export RUSTFLAGS="-C target-feature=+simd128"
wasm-pack -v build \
    --out-dir www/pkg \
    --target no-modules \
    --mode no-install
