#!/bin/bash

# steps to build the image 

# new machine of ubuntu 2204 lts have google connectivity
# such as a azure us server. or proxy . both docker-host and docker-container.
# use host proxy :: docker run -it --name ubt2204 --network=host ubuntu:22.04

su root
apt update 
apt install docker.io 
cd ~ 
git clone https://github.com/TaxLintDAO/taxlint.git --depth=1
cd taxlint/Reproducible
docker build -t taxlint:v1 .
docker build -t taxlint:v1 . --network=host


