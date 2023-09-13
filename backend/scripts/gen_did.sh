#!/bin/bash

# IMPORTANT buggy dont run . mannul run !
echo '==============================='
echo 'cargo build --target wasm32-unknown-unknown --release --package "backend"  --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm"  > ./backend/backend.did  --allow-precompiled 
'

# buggy if move .sh file location cause by ./  !!!! 
# TODO: need get a project root location for all script !
cargo build --target wasm32-unknown-unknown --release --package "backend"  --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm"  > ./backend/backend.did  --allow-precompiled 
# wasmtime: runtime for wasm

