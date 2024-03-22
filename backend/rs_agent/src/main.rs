use candid::{CandidType, Decode, Encode, Nat, Principal};
#[allow(unused_imports)]
#[allow(unused_imports)]
use ic_utils::call::AsyncCall;

use chrono::prelude::*;
use std::borrow::Borrow;
use std::fs::{read, File};
use std::io::Read;
use std::{env, fs};

// run canister_on_ic update :
//  ./target/debug/rs_agent 1
// run canister_on_local update :
//  ./target/debug/rs_agent 0
#[tokio::main]
async fn main() {
  regular_update_canister_with_db().await;
}

async fn regular_update_canister_with_db() {
  let (canister_id, agent, online_mode) = init_agent().await;
  let ic_or_local;
  if online_mode == "0" {
    ic_or_local = "local"
  } else if online_mode == "1" {
    ic_or_local = "ic"
  } else {
    panic!("err mode ic or local 1,0")
  }

  // ! save payload to dev machine file and read to rust
  let now = Local::now().format("%Y%m%d%H%M%S").to_string();
  let payload =
    collect_running_payload_simple(agent.borrow(), canister_id).await;
  save_payload_to_local(payload, now.clone(),ic_or_local.to_owned());
  let payload_with_time_tag = read_db_from_local(now,ic_or_local.to_owned());

  // ! deploy ic
  // FIXME. run ok. but not displaying cmds in real time .
  let _ = exec_deploy(ic_or_local.to_owned()).await;

  // ! send payload to ic and set payload on ic
  let args = candid::encode_one(payload_with_time_tag).unwrap();
  let result = send_payload_and_set(agent.borrow(), canister_id, args).await;
  println!("{}", result);
}

fn save_payload_to_local(payload: String, time_tag: String,mode:String) {
  let filename =
    format!("/home/btwl/code/ic/tax_lint/db/{}/payload_{}.json", time_tag,mode);
  let contents = payload;

  fs::write(filename, contents).expect("Unable to write file");
}

async fn init_agent() -> (Principal, Agent, String) {
  let controller = String::from("btwlz");
  let args: Vec<String> = env::args().collect();
  let online_mode = &args[1];

  let url_local = String::from("https://127.0.0.1:40010");
  let url_ic = String::from("https://ic0.app/");

  let canister_id_local =
    Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
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

  // !INFO this need use input passwd in terminal if have passwd. takes about 3s
  // to run
  let identity = get_dfx_identity(&controller);
  let agent = build_ic_agent(url, identity).await;
  (canister_id, agent, online_mode.to_string())
}

async fn send_payload_and_set(
  agent: &Agent,
  canister_id: Principal,
  my_arg: Vec<u8>,
) -> String {
  let response = agent
    .update(&canister_id, "set_payload_using_dev_machine_file")
    .with_arg(my_arg)
    .call_and_wait()
    .await;
  let result = candid::decode_one(&response.unwrap()).unwrap();
  result
}

async fn collect_running_payload_simple(
  agent: &Agent,
  canister_id: Principal,
) -> String {
  let response = agent
    .query(&canister_id, "collect_running_payload_simple")
    .with_arg(candid::encode_one(()).unwrap())
    .call()
    .await;
  let res = &response.expect("agent err");
  let result: String = candid::decode_one(res).unwrap();
  result
}

async fn greet_test(agent: Agent, canister_id: Principal) -> String {
  let response = agent
    .query(&canister_id, "greet_test")
    .with_arg(candid::encode_one(()).unwrap())
    .call()
    .await;
  match response {
    Ok(data) => {
      let ret = String::from_utf8(data).unwrap();
      return ret;
    }
    Err(e) => {
      let ret = format!("####An error occurred: {:?}", e);
      return ret;
    }
  }
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

fn read_db_from_local(time_tag: String,mode :String) -> String {
  let mut file = File::open(format!(
    "/home/btwl/code/ic/tax_lint/db/{}/payload_{}.json", time_tag,mode
  ))
  .expect("Unable to open the file");
  let mut db_json = String::new();
  file
    .read_to_string(&mut db_json)
    .expect("Unable to read the file");
  db_json
}

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

async fn exec_deploy(ic_or_local: String) -> std::io::Result<()> {
  let mut child =
    Command::new("/home/btwl/code/ic/tax_lint/backend/scripts/deploy_backend")
      .arg(ic_or_local)
      .stdout(Stdio::piped())
      .spawn()?;

  let stdout = child.stdout.take().unwrap();
  let reader = BufReader::new(stdout);

  for line in reader.lines() {
    println!("{}", line?);
  }

  let status = child.wait()?;
  println!("command exited with: {}", status);
  Ok(())
}
