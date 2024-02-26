#!/bin/bash
#### the os running this script need: 
# assume got http_proxy ok on host-machine : container can use it .
####!!!! this proxy only work at current terminal everytime. 
# docker run -it --name ubt2204 --network=host -e http_proxy=http://127.0.0.1:25526 -e https_proxy=http://127.0.0.1:25526 ubuntu:22.04
# apt update && apt install vim -y && cd ~ 

apt update && apt install git curl  -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
apt-get install build-essential cmake -y
# this downlaod master branch.
git clone https://github.com/TaxLintDAO/taxlint.git --depth=1 && cd taxlint 


cargo check && cargo build --target wasm32-unknown-unknown --release -p backend --locked
echo -e "\033[32m if see wasm file. then dev_env ready! \n \033[0m"
la ./target/wasm32-unknown-unknown/release/backend.wasm

#### optional: install and runs dfx commands.
# sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
# dfx --version
# dfx start --background
# dfx deploy backend
