#!/bin/bash
wget https://github.com/dfinity/pocketic/releases/download/3.0.1/pocket-ic-x86_64-linux.gz 
sudo gzip -d  pocket-ic-x86_64-linux.gz 
mv pocket-ic-x86_64-linux pocket-ic
mv pocket-ic /opt/pocket-ic
la  /opt/pocket-ic
pocket-ic --version

