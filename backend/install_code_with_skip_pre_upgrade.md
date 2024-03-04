```bash
(record {
      canister_id : canister_id;
      num_requested_changes : opt nat64;
  })



dfx canister \
--ic \ 
call aaaaa-aa install_code\
 '(record {
  canister_id = principal "v7g7o-oiaaa-aaaag-qcj3q-cai"; 
  mode = variant { upgrade = opt record { skip_pre_upgrade = opt true} };
  wasm_module = /home/btwl/code/ic/tax_lint/target/wasm32-unknown-unknown/release/backend.wasm;
  arg =  "???"
    })'\
  --wallet $(dfx identity --ic get-wallet) \
  --candid /home/btwl/code/ic/tax_lint/backend/canisters/backend/my_tests/test_in_cmd/dfx_calls/manage_can.did

dfx canister \
call aaaaa-aa install_code\
 '(record {
  mode = variant { upgrade = opt record { skip_pre_upgrade = opt true} };
  canister_id = principal "be2us-64aaa-aaaaa-qaabq-cai"; 
  wasm_module =  blob "0x0061736d01000000";
  arg =  blob "0x00";
    })'\
  --wallet $(dfx identity  get-wallet) \
  --candid /home/btwl/code/ic/tax_lint/backend/canisters/backend/my_tests/test_in_cmd/dfx_calls/manage_can.did
  
dfx canister \
call aaaaa-aa install_code\
 '(record {
  mode = variant { upgrade = opt record { skip_pre_upgrade = opt true} };
  canister_id = principal "be2us-64aaa-aaaaa-qaabq-cai"; 
  wasm_module =  blob "0x0061736d01000000";
  arg =  blob "0x00";
    })'\
  --wallet $(dfx identity  get-wallet) \
  --candid /home/btwl/code/ic/tax_lint/backend/canisters/backend/my_tests/test_in_cmd/dfx_calls/manage_can.did

  install_code : (record {
    mode : variant {
      install;
      reinstall;
      upgrade : opt record {
        skip_pre_upgrade: opt bool;
      }
    };
    canister_id : canister_id;
    wasm_module : wasm_module;
    arg : blob;
    sender_canister_version : opt nat64;
  }) -> ();


record {
  arg : vec nat8;
  wasm_module : vec nat8;
  mode : variant {
    reinstall;
    upgrade : record { skip_pre_upgrade : bool };
    install;
  };
  canister_id : principal;
} 

(record {
    mode : variant {
      install;
      reinstall;
      upgrade : opt record {
        skip_pre_upgrade: opt bool;
      }
    };
    canister_id = principal "v7g7o-oiaaa-aaaag-qcj3q-cai";
    wasm_module : wasm_module;
    arg : blob;
    sender_canister_version : opt nat64;
  })
```