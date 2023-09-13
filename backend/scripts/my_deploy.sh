#!/bin/bash

# IMPORTANT buggy dont run . mannul run !

cargo build --target wasm32-unknown-unknown --release --package "backend"      --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm"  > ./backend/backend.did  --allow-precompiled 
# wasmtime: runtime for wasm


# PLZ PLZ ignore that shit: always stuck me due to network !my stupid perfectionism! 
# WARN: Cannot check for vulnerabilities in rust canisters because cargo-audit is not installed. Please run 'cargo install cargo-audit' so that vulnerabilities can be detected.
dfx deploy backend --network ic

base_dir=$(pwd)
cd ./.dfx/ic/canisters/backend
rm backend.did
rm backend.did.d.ts
rm backend.did.js

# above cmd with passwd, run this script:
cd $base_dir
#   mainly run : dfx deploy backend --network ic   #with passwd.
./backend/scripts/exec_dfx_with_passwd.exp 

cd ./.dfx/ic/canisters/backend
# exec this:
mv service.did backend.did
mv service.did.d.ts backend.did.d.ts
mv service.did.js backend.did.js

cd $base_dir
git add -f ./.dfx/ic/canisters/backend/backend.did 
git add -f ./.dfx/ic/canisters/backend/backend.did.d.ts  
git add -f ./.dfx/ic/canisters/backend/backend.did.js 
git add -f ./.dfx/ic/canisters/backend/index.js 

# git add -f ./.dfx/local/canisters/backend/backend.did
# git add -f ./.dfx/local/canisters/backend/backend.did.d.ts  
# git add -f ./.dfx/local/canisters/backend/backend.did.js
# git add -f ./.dfx/local/canisters/backend/index.js

dfx canister status --network ic --wallet vwfus-yaaaa-aaaag-qcj2a-cai backend

