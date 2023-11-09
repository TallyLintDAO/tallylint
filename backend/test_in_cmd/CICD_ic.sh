#!/bin/bash
#! Continious Deploy on Main net :
# step1: gen did
# cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did 
# above work for ic-cdk 0.10.0
# 0.11.3 use this :
cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/backend.did
# maybe almost the same stuff. just abstraction or simplfied 0.10.0 cmd.
# step2:
dfx deploy backend --network ic 
# or  dfx deploy backend --network ic  -m reinstall  #this will empty the ic-DB 
# step3 : run  at project root 
./change_name.sh
# step4: git push did file to front dev. 
./backend/scripts/sync_remote.sh "query wallet-records api working "
#todo: maybe use makefile or bash can auto this process.