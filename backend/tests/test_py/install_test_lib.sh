#!/bin/bash
sudo cd /opt
curl  https://download.dfinity.systems/ic/307d5847c1d2fe1f5e19181c7d0fcec23f4658b3/openssl-static-binaries/x86_64-linux/pocket-ic.gz 
gzip -d  pocket-ic.gz
sudo chmod 777 ./pocket-ic
echo 'POCKET_IC_BIN =/opt/pocket-ic' >> /etc/enviroment
sudo pip3 install pocket-ic

