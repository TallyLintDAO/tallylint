






 dfx canister call backend add_wallet '(record { addr = "01awd916dwa335wda2042"; name = "AmydaLu"; w_type = "asdaw" })'
 dfx canister call backend add_wallet '(record { addr = "addr111"; name = "astrome001"; w_type = "astrome" })'
 dfx canister call backend add_wallet '(record { addr = "adr333"; name = "astrome001"; w_type = "astrome" })'
 dfx canister call backend delete_wallet "01awd916dwa335wda2042"
 dfx canister call backend query_wallet_array


  dfx canister call backend add_wallet '(record { addr = "test_dup001"; name = "astrome001"; w_type = "astrome" })'


dfx canister call --network ic backend add_wallet '(record { addr = "01awd916dwa335wda2042"; name = "AmydaLu"; w_type = "asdaw" })'
dfx canister call --network ic backend add_wallet '(record { addr = "addr111"; name = "astrome001"; w_type = "astrome" })'
dfx canister call --network ic backend add_wallet '(record { addr = "adr333"; name = "astrome001"; w_type = "astrome" })'
dfx canister call --network ic backend delete_wallet "01awd916dwa335wda2042"
dfx canister call --network ic backend query_wallet_array
dfx canister call --network ic backend add_wallet '(record { addr = "test_dup001"; name = "astrome001"; w_type = "astrome" })'