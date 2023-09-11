#!/bin/bash
cargo build --target wasm32-unknown-unknown --release --package "backend"      --features "ic-cdk/wasi"

wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm"  > ./backend/backend.did  --allow-precompiled