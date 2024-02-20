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
# cargo build --target wasm32-unknown-unknown --release -p backend --locked
# deprecated:
# cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did
dfx deploy backend 

#TODO: effiency: maybe use makefile or bash can auto this process.
#! Continious Deploy on Main net :
# step1: gen did
# cargo build --target wasm32-unknown-unknown --release --package "backend" --features "ic-cdk/wasi" && wasmtime "./target/wasm32-unknown-unknown/release/backend.wasm" --allow-precompiled >./backend/backend.did 
# above work for ic-cdk 0.10.0
# 0.11.3 use this :
cargo build --release --target wasm32-unknown-unknown --package backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >./backend/canisters/backend/backend.did
# cargo build --target wasm32-unknown-unknown --release -p backend --locked

# maybe almost the same stuff. just abstraction or simplfied 0.10.0 cmd.
# step2:
dfx deploy backend --network ic 
# or  dfx deploy backend --network ic  -m reinstall  #this will empty the ic-DB 
# step3 : run  at project root 
./change_name.sh
# step4: git push did file to front dev. 
./backend/scripts/sync_remote.sh "did uploading of sync records"

dfx identity use btwl0
dfx deploy backend --network ic

# IMPORTANT
# TDD idea , Test Driven Development . a kind of OKR .  to confident with  code !
# TODO. Itest autorun: make the whole CRUD into a auto things . auto-test
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


# good wallet addr with proper transactions  
dfx canister call backend add_wallet '(record { address = "107b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"; name = "AmydaLu"; from = "asdaw" })'

#neuron CRUD test 
dfx canister call backend auto_register_user 
dfx canister call backend add_neuron_wallet '(record { address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"; name = "AmydaLu"; from = "asdaw" })'
dfx canister call backend add_neuron_wallet '(record { address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf755"; name = "A2"; from = "nns" })'
dfx canister call backend query_all_neuron_wallet
dfx canister call backend  update_neuron_wallet '(record { id = 10002; from="nns1"; name = "cczz";})'
dfx canister call backend  query_a_neuron_wallet 10002
dfx canister call backend  delete_neuron_wallet 10002
dfx canister call backend query_all_neuron_wallet
dfx canister call backend get_payload
dfx canister call backend set_payload
dfx canister call backend get_payload_from_stable_mem
dfx canister call backend set_stable_mem_use_payload
dfx canister call backend user_quantity --network ic 

# save to web2 using http
dfx canister call backend store_paylaod_to_dropbox 

# test http 
dfx canister call backend get_icp_usd_exchange 
dfx canister call backend save_payload_to_dropbox 
dfx canister call backend save_payload_to_dropbox_blocking 

# dropbox test:
```
curl -X POST https://content.dropboxapi.com/2/files/upload_session/append_v2 \
    --header "Authorization: Bearer sl.BuuId_E7l6mc-bxLmS06Tp6LfSr7MZKwnGP9QOYbrCVMZUpGVkthA0ZOPickGyFgW4R-5yP8M2c_R48rie99nmaEP_DsMTOwI52S8kmzR4cWwTbF5D-U5V73uzkrw1f2hlzB9D723MkIvgIz_vnbrvE" \
    --header "Dropbox-API-Arg: {\"close\":false,\"cursor\":{\"offset\":0,\"session_id\":\"1234faaf0678bcde\"}}" \
    --header "Content-Type: application/octet-stream" \
    --data-binary @local_file.txt
 sl.BuyKjfKtkY5uYGS0Rpgcsf48_EsXHSldu1jYObvkAPfXzLxaMYBRHTFboyymcWd5cncBsD1sXE5HlnSc0h6lsmtBwSVQXypYPfysA-P9PhfwLKrjUHkuOYlzf2QzGxrv1dUaoimjLYib-btGmCWYdus

# below test ok . if auth no expire. and file not name conflict same .
curl -X POST https://content.dropboxapi.com/2/files/upload \
    --header "Authorization: Bearer sl.Bv39QbKVzSxOkNxeII8sGQ4Sk_TgBn9njNWxwRdOjxZ-rnd0GTc0LJEKZWOIxCm5tSsttw3c7o-yxenyahHVA2T6ZzflHiOrVjFAGjwRF64iNwW35KQlyaUG1Cj80LXLtJynWBooOv9FYM26FeZq0js" \
    --header "Dropbox-API-Arg: {\"autorename\":false,\"mode\":\"add\",\"mute\":false,\"path\":\"/Homework/math/Matrices3.txt\",\"strict_conflict\":false}" \
    --header "Content-Type: application/octet-stream" \
    --data-binary @/home/btwl/code/ic/tax_lint/test.json

curl -X POST https://content.dropboxapi.com/2/files/download \
    --header "Authorization: Bearer sl.Bv39QbKVzSxOkNxeII8sGQ4Sk_TgBn9njNWxwRdOjxZ-rnd0GTc0LJEKZWOIxCm5tSsttw3c7o-yxenyahHVA2T6ZzflHiOrVjFAGjwRF64iNwW35KQlyaUG1Cj80LXLtJynWBooOv9FYM26FeZq0js" \
    --header "Dropbox-API-Arg: {\"path\":\"/Homework/math/Matrices3.txt\"}"



```






#neuron CRUD test 
dfx canister call backend auto_register_user  --network ic
dfx canister call backend add_neuron_wallet '(record { address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"; name = "AmydaLu"; from = "asdaw" })' --network ic
dfx canister call backend add_neuron_wallet '(record { address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d25626f655"; name = "A2"; from = "nns" })' --network ic
dfx canister call backend query_all_neuron_wallet --network ic
dfx canister call backend  update_neuron_wallet '(record { id = 10002; from="nns1"; name = "cczz";})' --network ic
dfx canister call backend  query_a_neuron_wallet 10002 --network ic
dfx canister call backend  delete_neuron_wallet 10002 --network ic




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


# mannually update db
--network ic
dfx canister call  backend do_pre_upgrade_and_print_db 
dfx canister call  backend do_pre_upgrade_and_print_db 
dfx canister call  backend do_post_upgrade 

 dfx canister call  backend do_post_upgrade ' {"id":10006,"users":[{"owner":"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae","name":"","create_time":1703043679175930023}],"wallets":[],"records":[],"neurons":[{"owner":"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae","name":"A2","id":10004,"address":"307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d25626f655","create_time":1703060944271573259,"update_time":1703060944271573259},{"owner":"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae","name":"A2","id":10003,"address":"307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf655","create_time":1703060897298639479,"update_time":1703060897298639479},{"owner":"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae","name":"A2","id":10002,"address":"307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf755","create_time":1703043683021244087,"update_time":0},{"owner":"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae","name":"A2","id":10005,"address":"307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf765","create_time":1703061030276576303,"update_time":1703061030276576303}]}'






 dfx canister call  backend do_post_upgrade   "{\"id\":10003,\"users\":[{\"own
er\":\"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae\",\"name\":\"\",\"create_time\":1703247739924712849}],\"w
allets\":[],\"records\":[],\"neurons\":[{\"owner\":\"b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae\",\"name\":
\"AmydaLu\",\"id\":10002,\"address\":\"307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739\",\"create_time\":170324
7749341550806,\"update_time\":1703247749341550806}]}asd"






# below test pass!!! 
#  step: clean db and restore using dropbox and call add function. ret is ok . not `not resigster yet`
# get short term token : https://www.dropbox.com/developers/apps/info/qi2656n62bhls4u
dfx canister call  backend save_payload_to_dropbox "sl.Bv2AeIHy2BD9tl_h-QySDyGNF3eniMMQD6rD_V5qDMv6kNkIO_h8-DKXY0nrRGZEKAiXnMqhaAxylmFzyiGTN8JZpZWQpGUOP9fWJhWmL26lxcPVG_yc7uA3v9sghWLKFKkctT7VxNXEgfSrEL2GlNA"

dfx canister call backend add_neuron_wallet '(record { address = "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf755"; name = "A2"; from = "nns" })'


# empty db
dfx canister call  backend restore_db_from_dropbox '("sl.Bv2AeIHy2BD9tl_h-QySDyGNF3eniMMQD6rD_V5qDMv6kNkIO_h8-DKXY0nrRGZEKAiXnMqhaAxylmFzyiGTN8JZpZWQpGUOP9fWJhWmL26lxcPVG_yc7uA3v9sghWLKFKkctT7VxNXEgfSrEL2GlNA", "2024_02_18_10_57_24")'

# registered user db  
dfx canister call  backend restore_db_from_dropbox '("sl.Bv2AeIHy2BD9tl_h-QySDyGNF3eniMMQD6rD_V5qDMv6kNkIO_h8-DKXY0nrRGZEKAiXnMqhaAxylmFzyiGTN8JZpZWQpGUOP9fWJhWmL26lxcPVG_yc7uA3v9sghWLKFKkctT7VxNXEgfSrEL2GlNA", "2024_02_18_11_24_41")'

#TODO important: do above save_payload_to_dropbox() and restore_db_from_dropbox() on product code.
# method1: run save_payload_to_dropbox() at preupgreade time .
# method2: goto online code version using git checkout. and add the save_payload_to_dropbox() to that version of code .

# TODO
#Next step . try to automate this test using code  ? (rust ? python ? )
# pocket-ic ? ic-agent ?  

#TODO need version control of prod-code running on ic.



Get the management canister interface and save it as aaaaa-aa.did. Then run

dfx canister --ic call aaaaa-aa canister_info '(record {canister_id = principal "v7g7o-oiaaa-aaaag-qcj3q-cai"; num_requested_changes = opt 5 : opt nat64})' --wallet $(dfx identity --ic get-wallet) --candid /home/btwl/code/ic/tax_lint/backend/canisters/backend/my_tests/test_in_cmd/dfx_calls/manage_can.did
There should be a list of the recent changes to the wasm included in the response


WARN: Cannot fetch Candid interface for canister_info, sending arguments with inferred types.

(
  record {
    controllers = vec {
      principal "vwfus-yaaaa-aaaag-qcj2a-cai";
      principal "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae";
    };
    module_hash = opt blob "\ba\d9\27\a5\39\cb\74\3c\1d\44\37\1c\2e\84\1c\86\6e\52\4a\80\3b\1d\b9\a4\c7\70\35\ab\15\c6\d7\4e";
    recent_changes = vec {
      record {
        timestamp_nanos = 1_701_315_812_304_149_149 : nat64; Thu Nov 30 2023 11:43:32 GMT+0800 (China Standard Time)
        canister_version = 1_252 : nat64;
        origin = variant {
          from_user = record {
            user_id = principal "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae";
          }
        };
        details = variant {
          code_deployment = record {
            mode = variant { upgrade };
            module_hash = blob "\79\73\d8\86\6d\e4\a7\b6\12\2c\f0\1d\1f\7e\4f\26\5d\1c\23\88\7a\65\40\3c\71\09\16\8f\90\8a\0a\61";
          }
        };
      };
      record {
        timestamp_nanos = 1_703_060_751_559_030_310 : nat64;
        canister_version = 1_494 : nat64;
        origin = variant {
          from_user = record {
            user_id = principal "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae";
          }
        };
        details = variant {
          code_deployment = record {
            mode = variant { reinstall };
            module_hash = blob "\1c\7b\71\d7\b1\3e\be\6b\d5\aa\84\f0\28\f0\1f\44\ed\ed\6c\19\f4\a4\77\26\8b\e3\98\ca\05\9c\ad\0a";
          }
        };
      };
      record {
        timestamp_nanos = 1_703_225_761_667_613_780 : nat64;
        canister_version = 1_513 : nat64;
        origin = variant {
          from_user = record {
            user_id = principal "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae";
          }
        };
        details = variant {
          code_deployment = record {
            mode = variant { reinstall };
            module_hash = blob "\1c\7b\71\d7\b1\3e\be\6b\d5\aa\84\f0\28\f0\1f\44\ed\ed\6c\19\f4\a4\77\26\8b\e3\98\ca\05\9c\ad\0a";
          }
        };
      };
      record {
        timestamp_nanos = 1_703_225_913_784_433_447 : nat64;
        canister_version = 1_516 : nat64;
        origin = variant {
          from_user = record {
            user_id = principal "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae";
          }
        };
        details = variant {
          code_deployment = record {
            mode = variant { upgrade };
            module_hash = blob "\fb\5b\3c\c3\5c\5d\82\38\69\a2\9b\67\5d\e2\e9\c3\41\ec\b6\21\b4\e3\7e\5c\2b\63\d8\cb\65\a5\e4\2d";
          }
        };
      };
      record {
        timestamp_nanos = 1_703_840_654_703_938_355 : nat64; Fri Dec 29 2023 17:04:14 GMT+0800 (China Standard Time) Thu Nov 30 2023 03:43:32 GMT+0000  -> commit hash: 6643435906e4fc1fc9868df1829069c14159fa9c
        canister_version = 1_604 : nat64;
        origin = variant {
          from_user = record {
            user_id = principal "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae";
          }
        };
        details = variant {
          code_deployment = record {
            mode = variant { upgrade };
            module_hash = blob "\ba\d9\27\a5\39\cb\74\3c\1d\44\37\1c\2e\84\1c\86\6e\52\4a\80\3b\1d\b9\a4\c7\70\35\ab\15\c6\d7\4e";
            # 0BAD927A539CB743C1D44371C2E841C866E524A803B1DB9A4C77035AB15C6D74E
          }
        };
      };
    };
    total_num_changes = 78 : nat64;
  },
)

git checkout -b prod_db_backup_2 6643435906e4fc1fc9868df1829069c14159fa9c


TODO: 逻辑上死亡卡住了在当前线上版本的preupgrade环节.  可能尝试management canister 等api 来获取当前 ic 上面的罐子的stable mem.
https://forum.dfinity.org/t/any-possibility-to-check-the-latest-wasm-code-install-time-on-main-ic-net/27682
TODO: 了解uninstall_code 的api是否会导致stable mem 被删除. 如果不会. 则选择force uninstall 然后install最新版本代码. **尤其注意install这个地方一定不要有对stable mem的任何写入**!

