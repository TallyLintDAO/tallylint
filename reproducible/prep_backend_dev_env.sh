#!/bin/bash
#### the os running this script need: 
# docker run -it --name ubt2204 --network=host ubuntu:22.04
# apt update && apt install wget -y 

apt update && apt install git curl  -y
curl https://sh.rustup.rs -sSf | sh -s -- -y
apt-get install build-essential cmake -y
source "$HOME/.cargo/env"
# this source cmd not work immediately in this bash sesssion of this script running time .
# have to run rest of code in another session
./continue_prep_env.sh.sh
