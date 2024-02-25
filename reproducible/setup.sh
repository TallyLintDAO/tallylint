#!/bin/bash

# reproducipuble build the project and generate codes-hash.

apt-get update && apt-get install -y nodejs npm
apt install curl
npm install -g n && n 18.17.1 && npm install -g npm@9.6.7
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
#   DFX_VERSION=0.14.3 sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup install 1.72.0 && rustup default 1.72.0
rustup target add wasm32-unknown-unknown wasm32-wasi x86_64-unknown-linux-gnu

####    2.build taxlint
cd ~ 
apt install git
git clone https://github.com/TaxLintDAO/taxlint.git
cd ./taxlint
npm install
npm build
cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did
dfx start --background
dfx deploy
touch local_module_hashes.txt
dfx canister status --all | grep "Module hash" | awk '{ print $3 }' >local_module_hashes.txt
# compare hash :
diff ./Reproducible/ic_module_hashes.txt ./local_module_hashes.txt

# assume got http_proxy ok on host-machine : container can use it .
# docker run -it --name ubt2204 --network=host ubuntu:22.04
####!!!! this proxy only work at current terminal everytime. 
# docker run -it --name ubt2204 --network=host -e http_proxy=http://127.0.0.1:25526 -e https_proxy=http://127.0.0.1:25526 ubuntu:22.04

#!/bin/bash
# apt update && apt install git curl  vim -y
git clone https://github.com/TaxLintDAO/taxlint.git --depth=1
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
curl https://sh.rustup.rs -sSf | sh -s -- -y

dfx --version
which curl 


cd taxlint 
dfx start --background
dfx deploy backend
