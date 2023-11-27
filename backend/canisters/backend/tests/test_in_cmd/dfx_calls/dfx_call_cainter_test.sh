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
# cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did 
# above work for ic-cdk 0.10.0
# 0.11.3 use this :
cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/backend.did
cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/canisters/backend/backend.did
cargo build --target wasm32-unknown-unknown --release -p backend --locked

# maybe almost the same stuff. just abstraction or simplfied 0.10.0 cmd.
# step2:
dfx deploy backend --network ic 
# or  dfx deploy backend --network ic  -m reinstall  #this will empty the ic-DB 
# step3 : run  at project root 
./change_name.sh
# step4: git push did file to front dev. 
./backend/scripts/sync_remote.sh "did uploading of sync records"
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
dfx canister call backend add_wallet '(record { address = "c5"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend update_wallet '(record { address = "c5"; name = "AmydaLu"; from = "asdaw";transactions=8; last_sync_time=8; last_transaction_time=9;})'
dfx canister call backend update_wallet '(record { address = "c7"; name = "AmydaLu"; from = "asdaw";transactions=8; last_sync_time=8; last_transaction_time=9;})'
# update a non exist wallet test :
dfx canister call backend update_wallet '(record { address = "c9"; name = "AmydaLu"; from = "asdaw";transactions=8; last_sync_time=8; last_transaction_time=9;})'
dfx canister call backend add_wallet '(record { address = "c9"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_wallet '(record { address = "c9"; name = "AmydaLu"; from = "asdaw" })'

#neuron CRUD test 
dfx canister call backend add_neuron_wallet '(record { address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_wallet '(record { address = "107b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"; name = "AmydaLu"; from = "asdaw" })'

dfx canister call backend query_all_neuron_wallet
dfx canister call backend query_a_wallet 10003
dfx canister call backend  update_neuron_wallet '(record { id = 10003; from="nns1"; name = "cczz";})'




# testing :    backend: http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
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
dfx canister call backend  update_wallet '(record { id = 10002; name = "cczz";})'
dfx canister call backend  update_neuron_wallet '(record { id = 10003; name = "cczz"; from="nns1";})'
dfx canister call backend  query_a_wallet "10006"
dfx canister call backend query_all_wallets --query
dfx canister call backend delete_transaction_record "10006"
dfx canister call backend add_transaction_record '(record {id=10006; tag="tg1"; time=12356; t_type="SEND"; comment="c1"; address="add1"; manual=true; price=10.0; amount=1})' 



dfx canister call --network ic backend auto_register_user
dfx canister call --network ic backend get_balance

dfx canister call --network ic backend add_wallet '(record { address = "0d7c6cff7cee0f5b34cc62f02bc0cfef7e5e744ecdd841edfd68256431d448e3"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call --network ic backend add_wallet '(record { address = "addr111"; name = "astrome001"; from = "astrome" })'
dfx canister call --network ic backend add_wallet '(record { address = "adr333"; name = "astrome001"; from = "astrome" })'
dfx canister call --network ic backend delete_wallet "01awd916dwa335wda2042"
dfx canister call --network ic backend query_all_wallets --query
dfx canister call --network ic backend get_caller_principal
dfx canister call --network ic backend add_wallet '(record { address = "test_dup001"; name = "astrome001"; from = "astrome" })'
dfx canister call --network ic backend query_all_wallets
dfx canister call --network ic backend query_a_wallet "10016"
dfx canister call --network ic backend delete_wallet "10016"
dfx canister call --network ic backend update_wallet "10016"
dfx canister call --network ic backend get_caller_principal
dfx canister call --network ic backend test_print
dfx canister call --network ic backend list_all_user
dfx canister call --network ic backend user_quantity
dfx canister call --network ic backend auto_register_user
dfx canister call --network ic backend  get_neuron_info "9758293084897065223"
dfx canister call --network ic backend  get_neuron_info "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"






command palette

