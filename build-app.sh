#!/bin/bash -e
echo "Building debug web targets..."
echo
rm app/infuse*
cargo build --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/infuse.wasm --out-dir app
