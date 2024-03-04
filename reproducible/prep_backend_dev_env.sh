#!/bin/bash
#### the os running this script need: 
# docker run -it --name ubt2204 --network=host ubuntu:22.04
# apt update && apt install wget -y 

apt update && apt install git curl  -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
apt-get install build-essential cmake -y
# this downlaod master branch.
git clone https://github.com/TaxLintDAO/taxlint.git --depth=1 && cd taxlint 

source "$HOME/.cargo/env"
cargo check && cargo build --target wasm32-unknown-unknown --release -p backend --locked
echo -e "\033[32m if see wasm file. then dev_env ready! \n \033[0m"
ls ./target/wasm32-unknown-unknown/release/backend.wasm

#### optional: install and runs dfx commands.
# ! this need user to hand input to set some config 
# sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
# dfx --version
# dfx start --background
# dfx deploy backend
