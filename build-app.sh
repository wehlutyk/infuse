#!/bin/bash -e
echo "Building debug web targets..."
echo
rm -f app/infuse.d.ts app/infuse.js app/infuse_bg.wasm
cargo build --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/infuse.wasm --out-dir app
