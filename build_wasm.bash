#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/yausg.wasm
cp index.html ./out/.
cd out
python -m http.server 8000
