#!/bin/bash
apt update && apt install git curl  vim -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
# sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
# this downlaod master branch.
git clone https://github.com/TaxLintDAO/taxlint.git --depth=1 && cd taxlint 

# dfx --version
which curl 

# dfx start --background
apt-get install build-essential cmake -y
clear && cargo check && cargo build --target wasm32-unknown-unknown --release -p backend --locked
echo 如果你看到了wasm 文件 就证明后端环境OK, 如果No such file or directory  环境有问题
la ./target/wasm32-unknown-unknown/release/backend.wasm
# if see wasm file. then dev_env ok!

