#!/bin/bash
cd ./backend/scripts/
./gen_did.sh
echo '==============================='
echo 'dfx deploy backend --network ic'
./exec_dfx_with_passwd.exp