#!/bin/bash
# new machine of ubuntu 2204 lts have google connectivity
# such as a azure us server. or proxy . both docker-host and docker-container.
# steps to build the image 
su root
apt update 
apt install docker.io 
cd ~ 
git clone https://github.com/TaxLintDAO/taxlint.git
cd taxlint/Reproducible
docker build -t taxlint:v1 .
docker build -t taxlint:v1 . --network=host


