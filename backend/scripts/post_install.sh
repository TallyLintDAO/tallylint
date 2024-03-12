#!/bin/bash
# TODO do this  and pre_install stage : 
echo ====pre_install task: candid-extractor====
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/canisters/backend/backend.did


dfx deploy backend --verbose
# DFXPASS_BTWLZ
echo ====post_install task: change interface file name====
base_dir=$(git rev-parse --show-toplevel)
echo "The project root directory is ${base_dir}"

cd $base_dir
cd ./.dfx/ic/canisters/backend
mv service.did backend.did
mv service.did.d.ts backend.did.d.ts
mv service.did.js backend.did.js

# mv会覆盖写入已存在的同名文件
cd $base_dir
cd ./.dfx/local/canisters/backend
mv service.did backend.did
mv service.did.d.ts backend.did.d.ts
mv service.did.js backend.did.js

cd $base_dir
git add -f ./.dfx/ic/canisters/backend/backend.did 
git add -f ./.dfx/ic/canisters/backend/backend.did.d.ts  
git add -f ./.dfx/ic/canisters/backend/backend.did.js 
git add -f ./.dfx/ic/canisters/backend/index.js 

cd $base_dir
git add -f ./.dfx/local/canisters/backend/backend.did
git add -f ./.dfx/local/canisters/backend/backend.did.d.ts  
git add -f ./.dfx/local/canisters/backend/backend.did.js
git add -f ./.dfx/local/canisters/backend/index.js

# TODO might need CI git auto commit and push .
