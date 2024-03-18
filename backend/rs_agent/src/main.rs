use candid::{Decode, Principal};
#[allow(unused_imports)]
#[allow(unused_imports)]
use ic_utils::call::AsyncCall;
use ic_utils::interfaces::management_canister::builders::InstallMode;
use ic_utils::interfaces::ManagementCanister;
use std::env;
use std::fs::read;

// sometime debug cache err:
// cargo clean -p canister_upgrader && cargo build --package canister_upgrader

// testing :
//  ./target/debug/canister_upgrader 0 1
#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();
  let online_mode = &args[1];
  let install_mode = &args[2];

  let url_local = String::from("https://127.0.0.1:40010");
  let url_ic = String::from("https://ic0.app/");

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
    panic!("args input err!!");
  }

  let controller = String::from("btwlz");

  // INFO this need use input passwd in terminal if have passwd. takes about 4s
  // to run
  let identity = get_dfx_identity(&controller);
  let agent = build_ic_agent(url, identity).await;
  let response = agent
    .update(&canister_id_local, "send_payload_string_to_canister")
    .with_arg(Encode!(&Argument { amount: None })?)
    .call_and_wait()
    .await?;
  let result = Decode!(response.as_slice(), String)?;
}

use ic_agent::agent::http_transport::reqwest_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::{Agent, Identity};

pub async fn build_ic_agent(url: String, identity: Box<dyn Identity>) -> Agent {
  let mainnet = is_mainnet(&url);
  let transport = ReqwestHttpReplicaV2Transport::create(url)
    .expect("Failed to create Reqwest transport");
  let timeout = std::time::Duration::from_secs(60 * 5);

  let agent = Agent::builder()
    .with_transport(transport)
    .with_boxed_identity(identity)
    .with_ingress_expiry(Some(timeout))
    .build()
    .expect("Failed to build IC agent");

  if !mainnet {
    let rk_path = "/home/btwl/code/canister_upgrader_independent2/btwlz_pk.pem";
    let rk = read(rk_path).expect("file not exsit");
    agent.set_root_key(rk);
  }

  agent
}
pub fn is_mainnet(url: &str) -> bool {
  url.contains("ic0.app")
}

pub fn get_dfx_identity(name: &str) -> Box<dyn Identity> {
  let logger = slog::Logger::root(slog::Discard, slog::o!());
  let mut identity_manager =
    dfx_core::identity::IdentityManager::new(&logger, &None).unwrap();
  identity_manager
    .instantiate_identity_from_name(name, &logger)
    .unwrap()
}
// ic_utils lib hot fix patch : git commit:
// b74445e1da0a6afefc3a08372f74e8ea416cd1ba

pub struct BuildVersion {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

pub struct MyArgs {
  pub wasm_version: BuildVersion,
}
