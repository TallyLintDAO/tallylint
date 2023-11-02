## backend ic address    
backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=v7g7o-oiaaa-aaaag-qcj3q-cai

## add cycles to can:
dfx canister --network ic --wallet vwfus-yaaaa-aaaag-qcj2a-cai deposit-cycles 5000000000000 assets
5 000 000 000 000 5TC

## generate rust backend canister did file :
https://internetcomputer.org/docs/current/developer-docs/backend/rust/candid



## check canister cycles balance:
 dfx canister status backend --network ic 



#! local deploy steps:
dfx start --background
cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did
dfx deploy backend 

#! Continious Deploy on Main net :
# step1: gen did
cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did 
# step2:
dfx deploy backend --network ic 
# or  dfx deploy backend --network ic  -m reinstall  #this will empty the ic-DB 
# step3 : run  at project root 
./change_name.sh
# step4: git push did file to front dev. 
#todo: maybe use makefile or bash can auto this process.

dfx identity use btwl0
dfx deploy backend --network ic

# IMPORTANT
# TDD idea , Test Driven Development . a kind of OKR .  to confident with  code !
# TODO. make the whole CRUD into a auto things . auto-test
# both work to local or ic need test .
dfx canister call backend auto_register_user

dfx canister call backend add_wallet '(record { address = "a1"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_wallet '(record { address = "c1"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_wallet '(record { address = "c3"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_wallet '(record { address = "c2"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_wallet '(record { address = "c5"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend update_wallet '(record { address = "c5"; name = "AmydaLu"; from = "asdaw";transactions=8; last_sync_time=8; last_transaction_time=9;})'
dfx canister call backend update_wallet '(record { address = "c7"; name = "AmydaLu"; from = "asdaw";transactions=8; last_sync_time=8; last_transaction_time=9;})'
# update a non exist wallet test :
dfx canister call backend update_wallet '(record { address = "c9"; name = "AmydaLu"; from = "asdaw";transactions=8; last_sync_time=8; last_transaction_time=9;})'
dfx canister call backend add_wallet '(record { address = "c9"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_transaction_record '(record { address = "c9"; name = "AmydaLu"; from = "asdaw" })'



dfx canister call backend delete_wallet 100002
dfx canister call backend query_all_wallets --query
dfx canister call backend get_caller_principal
dfx canister call backend test_print
dfx canister call backend list_all_user
dfx canister call backend user_quantity
dfx canister call backend auto_register_user
dfx canister call backend create_and_install
dfx canister call backend whoami
dfx canister call backend  get_canister_info "c2lt4-zmaaa-aaaaa-qaaiq-cai"
dfx canister call backend  get_canister_status "c2lt4-zmaaa-aaaaa-qaaiq-cai"
dfx canister call backend  get_neuron_info "9758293084897065223"
dfx canister call backend  delete_wallet "9758293084897065223"
dfx canister call backend  query_a_wallet "10002"
dfx canister call backend  query_a_wallet "10006"
dfx canister call backend query_all_wallets --query


dfx canister call --network ic backend auto_register_user

dfx canister call --network ic backend add_wallet '(record { address = "01awd916dwa335wda2042"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call --network ic backend add_wallet '(record { address = "addr111"; name = "astrome001"; from = "astrome" })'
dfx canister call --network ic backend add_wallet '(record { address = "adr333"; name = "astrome001"; from = "astrome" })'
dfx canister call --network ic backend delete_wallet "01awd916dwa335wda2042"
dfx canister call --network ic backend query_all_wallets --query
dfx canister call --network ic backend get_caller_principal
dfx canister call --network ic backend add_wallet '(record { address = "test_dup001"; name = "astrome001"; from = "astrome" })'
dfx canister call --network ic backend query_wallet_array
dfx canister call --network ic backend get_caller_principal
dfx canister call --network ic backend test_print
dfx canister call --network ic backend list_all_user
dfx canister call --network ic backend user_quantity
dfx canister call --network ic backend auto_register_user
dfx canister call --network ic backend  get_neuron_info "9758293084897065223"
