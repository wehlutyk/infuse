#!/bin/bash -e
echo "Building debug web targets..."
echo
rm -f app/infuse*
cargo build --target=wasm32-unknown-unknown

echo
echo -e "\033[1;32m  Processing\033[0m running wasm-bindgen"
wasm-bindgen target/wasm32-unknown-unknown/debug/infuse.wasm --out-dir app
echo -e "\033[1;32m    Finished\033[0m wasm-bindgen"
