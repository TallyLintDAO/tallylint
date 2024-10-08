#!/bin/bash

# this downlaod master branch.
git clone https://github.com/TallyLintDAO/tallylint.git --depth=1 && cd tallylint
cargo install candid-extractor
cargo check && cargo build --target wasm32-unknown-unknown --release -p backend --locked
echo -e "\033[32m #### if you see wasm file below. then dev_env is ready! #### \n \033[0m"
ls ./target/wasm32-unknown-unknown/release/backend.wasm

#### optional: install and runs dfx commands.
# ! this need user to hand input to set some config 
# sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
# dfx --version
# dfx start --background
# dfx deploy backend

