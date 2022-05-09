#!/bin/bash
set -eux

wasm-pack build --out-dir www/pkg --target no-modules
