use std::fs::read;

use client::setup::{setup_new_env, CanisterId};

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
pub mod backend_test;
pub mod client;

fn main() {
  // let env = setup_new_env(None);
  // use backend::*;
  // let TestData { user1, group_id, .. } = init_test_data(env,
  // canister_ids.local_user_index);

  // generate_update_call!(sync_transaction_record);
  //     let sync_transaction_record_response =
  // client::user::sync_transaction_record(       env,
  //       user1.principal,
  //       user1.canister(),
  //       &user_canister::delete_group::Args { chat_id: group_id },
  //   );
  test_counter_canister();
}

use candid::{encode_one, Principal};
use pocket_ic::{PocketIc, WasmResult};
use rand::random;
use serde_bytes::ByteBuf;

use crate::client::setup::{Cycles, TERA};

fn test_counter_canister() {
  let pic = PocketIc::new();
  // Create an empty canister as the anonymous principal and add cycles.
  let canister_id = pic.create_canister();

  pic.add_cycles(canister_id, 2 * TERA);

  let wasm_file_path = "/home/btwl/code/ic/tax_lint/target/wasm32-unknown-unknown/release/backend.wasm";
  let wasm_bytes = read(wasm_file_path).expect("wasm file not exsit");

  pic.install_canister(canister_id, wasm_bytes, vec![], None);
  let user1 = random_user_principal().0;

  call_canister(&pic, canister_id, user1, "auto_register_user");
  let reply = call_canister(&pic, canister_id, user1, "greet_test");
  let ret = String::from_utf8(wasm_result_to_vec8(reply)).unwrap();
  println!("{}", ret);
}
fn wasm_result_to_vec8(result: WasmResult) -> Vec<u8> {
  match result {
    WasmResult::Reply(vec) => vec,
    WasmResult::Reject(err) => err.into_bytes(),
  }
}

fn call_canister(
  pic: &PocketIc,
  canister_id: CanisterId,
  sender: Principal,
  method: &str,
) -> WasmResult {
  pic
    .update_call(canister_id, sender, method, encode_one(()).unwrap())
    .expect("Failed to call counter canister")
}

const NNS_INTERNET_IDENTITY_CANISTER_ID: CanisterId =
  Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 10, 1, 1]);
pub fn random_user_principal() -> (Principal, ByteBuf) {
  let algorithm_bytes = [
    48u8, 60, 48, 12, 6, 10, 43, 6, 1, 4, 1, 131, 184, 67, 1, 2, 3, 44, 0,
  ];
  let random_bytes: [u8; 32] = random();

  let mut public_key = Vec::from(algorithm_bytes);
  public_key.push(NNS_INTERNET_IDENTITY_CANISTER_ID.as_slice().len() as u8);
  public_key.extend_from_slice(NNS_INTERNET_IDENTITY_CANISTER_ID.as_slice());
  public_key.extend_from_slice(&random_bytes);

  (
    Principal::self_authenticating(&public_key),
    ByteBuf::from(public_key),
  )
}

#[derive(Debug)]
pub struct CanisterIds {
  pub backend: CanisterId,
}
