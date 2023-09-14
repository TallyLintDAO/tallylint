#!/bin/bash


base_dir=$(pwd)
cd ./.dfx/ic/canisters/backend
rm backend.did
rm backend.did.d.ts
rm backend.did.js

# above cmd with passwd, run this script:
cd $base_dir

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