#!/bin/sh
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/ulid-wasm.wasm --out-dir ./pkg --target no-modules
