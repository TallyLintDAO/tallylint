


cargo build --target wasm32-unknown-unknown --release --package "backend"  --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm"  > ./backend/backend.did  --allow-precompiled 

# IMPORTANT
# TDD idea , Test Driven Development . a kind of OKR . great! to confident with our code !
# TODO. make the whole CRUD into a auto things !!!
# both work to local or ic need test !!!!!!
dfx canister call  backend auto_register_user

 dfx canister call backend add_wallet '(record { address = "a1"; name = "AmydaLu"; from = "asdaw" })'
 dfx canister call backend add_wallet '(record { address = "c1"; name = "AmydaLu"; from = "asdaw" })'
 dfx canister call backend add_wallet '(record { address = "c3"; name = "AmydaLu"; from = "asdaw" })'
 dfx canister call backend add_wallet '(record { address = "c2"; name = "AmydaLu"; from = "asdaw" })'
 dfx canister call backend delete_wallet 100002
 dfx canister call backend query_all_wallets --query
 dfx canister call backend get_caller_principal
 dfx canister call backend test_print
 dfx canister call backend list_all_user
 dfx canister call backend user_quantity
 dfx canister call backend auto_register_user







dfx canister call --network ic backend auto_register_user

dfx canister call --network ic backend add_wallet '(record { address = "01awd916dwa335wda2042"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call --network ic backend add_wallet '(record { addr = "addr111"; name = "astrome001"; w_type = "astrome" })'
dfx canister call --network ic backend add_wallet '(record { addr = "adr333"; name = "astrome001"; w_type = "astrome" })'
dfx canister call --network ic backend delete_wallet "01awd916dwa335wda2042"
dfx canister call --network ic backend query_wallet_array
dfx canister call --network ic backend get_caller_principal
dfx canister call --network ic backend add_wallet '(record { addr = "test_dup001"; name = "astrome001"; w_type = "astrome" })'

 dfx canister call --network ic backend query_wallet_array
 dfx canister call --network ic backend get_caller_principal
 dfx canister call --network ic backend test_print
 dfx canister call --network ic backend list_all_user
 dfx canister call --network ic backend user_quantity
 dfx canister call --network ic backend auto_register_user


