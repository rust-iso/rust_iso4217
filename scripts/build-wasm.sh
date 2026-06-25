#!/usr/bin/env bash
#
# Build the WebAssembly / npm package with wasm-pack.
#
# The library gates its `#[wasm_bindgen]` code paths behind
# `#[cfg(target_arch = "wasm32")]`, so wasm-pack activates them automatically
# when targeting wasm32 — no extra RUSTFLAGS are required. The JS bindings and
# the .wasm artifact are written to ./pkg, and wasm-pack syncs pkg/package.json
# from Cargo.toml (version, description, license, keywords).
#
# Usage:
#   scripts/build-wasm.sh [extra wasm-pack args]
#
set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "error: wasm-pack not found. Install it with: cargo install wasm-pack" >&2
    exit 1
fi

wasm-pack build --target web --release "$@"
