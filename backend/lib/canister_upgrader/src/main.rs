use candid::Principal;
use canister_agent_utils::{build_ic_agent, get_dfx_identity};
#[allow(unused_imports)]
use ic_agent::{Agent, Identity};
#[allow(unused_imports)]
use ic_utils::call::AsyncCall;
use ic_utils::interfaces::management_canister::builders::InstallMode;
use ic_utils::interfaces::ManagementCanister;
use std::env;
use std::fs::read;

// sometime debug cache err: cargo clean -p canister_upgrader && cargo build --package canister_upgrader
#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();
  let online_mode = &args[1];

  let url_local = String::from("https://127.0.0.1:40010");
  let url_ic = String::from("https://ic0.app/");
  // INFO this is local or ic canister_id
  let canister_id_local =
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap();
  let canister_id_ic =
    Principal::from_text("v7g7o-oiaaa-aaaag-qcj3q-cai").unwrap();
  let url;
  let canister_id;

  if online_mode == "0" {
    println!("local network mode");
    url = url_local;
    canister_id = canister_id_local;
  } else if online_mode == "1" {
    println!("ic network mode");
    url = url_ic;
    canister_id = canister_id_ic;
  } else {
      panic!("args input err!");
  }


  let controller = String::from("btwlz");

  // INFO this need use input passwd in terminal if have passwd.
  let identity = get_dfx_identity(&controller);
  let agent = build_ic_agent(url, identity).await;
  let management_canister = ManagementCanister::create(&agent);

  let wasm_file_path = "/home/btwl/code/ic/tax_lint/target/wasm32-unknown-unknown/release/backend.wasm";
  let wasm_bytes = read(wasm_file_path).expect("file not exsit");

  match management_canister
    .install_code(&canister_id, &wasm_bytes)
    .with_mode(InstallMode::Upgrade {
      skip_pre_upgrade: true, // Some(true)
    })
    .call_and_wait()
    .await
  {
    Ok(_) => println!("Wasm upgraded with skip_pre_upgrade ! "),
    Err(error) => println!("Upgrade failed: {error:?}"),
  };
}

// ic_utils lib hot fix patch : git commit:
// b74445e1da0a6afefc3a08372f74e8ea416cd1ba




