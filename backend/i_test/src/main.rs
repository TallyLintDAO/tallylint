use std::{any::Any, fs::read};

use client::setup::CanisterId;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub mod backend_test;
pub mod client;

fn main() {
  // test_query_transactions();
  // test_query_transactions2();
}

use candid::{encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, UserError, WasmResult};
use rand::random;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::client::setup::TERA;

fn test_query_transactions() {
  let env = PicEnv::new();
  let pic = env.pic;
  let canister_id = env.canister_id;
  let user1 = random_user_principal().0;

  // !clear env ?  maybe need to unsintall canister mannually  to clear stable
  // mem.

  // !register
  // let reply = call_canister(&pic, canister_id, user1, "auto_register_user");
  // let ret = String::from_utf8(wasm_result_to_vec8(reply)).unwrap();
  // println!("{}", ret);

  // !add_wallet
  let args = WalletAddCommand {
    address: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
      .to_string(),
    principal_id: None, /* This is set to None as per your structure. You
                         * might need to set it as per your requirements. */
    from: "NNS".to_string(),
    name: "w1".to_string(),
  };
  let ret = pic.update_call(
    canister_id,
    user1,
    "add_wallet",
    candid::encode_one(args).unwrap(),
  );

  let ret: Result<bool, String> = unwrap_response(ret);

  match ret {
    Ok(_) => println!("ok"),
    Err(_) => println!("err"),
  }
  // !query wallet info
  // generate_update_call!(query_all_wallets);

  // !add transactions
  // '(vec {record {123; vec {record {hash="123"; walletName="asd";
  // t_type="asd"; timestamp=123.0; details=record {to="asd"; fee=123.8;
  // status="asd"; ledgerCanisterId="asd"; value=1.0; cost=1.0; from="12";
  // currency=record {decimals=13; symbol="asd"}; profit=12.0; price=12.0;
  // amount=12.0}}}}})'
  // generate_update_call!(sync_transaction_record);

  // !query transactions
  // generate_query_call!(query_wallet_transactions);
}
fn test_query_transactions2() {
  let pic_env = PicEnv::new();
  let user1 = Principal::anonymous();

  let args = WalletAddCommand {
    address: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
      .to_string(),
    principal_id: None, /* This is set to None as per your structure. You
                         * might need to set it as per your requirements. */
    from: "NNS".to_string(),
    name: "w1".to_string(),
  };

  let ret: Result<bool, String> =
    pic_env.my_update_call(user1, args, "add_wallet");

  match ret {
    Ok(_) => println!("ok"),
    Err(_) => println!("err"),
  }
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

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct WalletAddCommand {
  pub address: String,
  pub principal_id: Option<String>, /* Plug use , need
                                     * to convert to
                                     * opt_account_id_hex for use. */
  pub from: String, /* from which wallet_type: such as
                     * NNS Plug  Stoic
                     * AstorMe  .. maybe add more */
  pub name: String,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct WalletProfile {
  // primary key
  pub id: u64,
  pub holder: Principal,

  // frontend para input
  pub address: String, // immutable.
  pub from: String,    /* from which wallet_type: such
                        * as  NNS Plug  Stoic
                        * AstorMe  .. maybe add more */
  pub name: String,

  pub principal_id: Option<String>, /* Plug use , need to convert to
                                     * opt_account_id_hex(address)
                                     * for use. */

  // backend auto-gen
  pub create_time: u64, //ic_cdk::api::time();

  pub transactions: u64, //transactions count
  pub last_sync_time: u64,
  pub last_transaction_time: u64,
}

pub struct PicEnv {
  pic: PocketIc,
  canister_id: Principal,
}

impl PicEnv {
  pub fn new() -> Self {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();

    pic.add_cycles(canister_id, 20 * TERA);

    let wasm_file_path = "/home/btwl/code/ic/tax_lint/target/wasm32-unknown-unknown/release/backend.wasm";
    let wasm_bytes = read(wasm_file_path).expect("wasm file not exsit");

    let _ = pic.reinstall_canister(canister_id, wasm_bytes, vec![], None);

    PicEnv { pic, canister_id }
  }

  fn my_update_call<
    ArgsType: candid::CandidType,
    ResponseType: candid::CandidType + DeserializeOwned,
  >(
    &self,
    user: Principal,
    args: ArgsType,
    method: &str,
  ) -> ResponseType {
    let ret_raw = self.pic.update_call(
      self.canister_id,
      user,
      method,
      candid::encode_one(args).unwrap(),
    );

    let ret: ResponseType = unwrap_response(ret_raw);
    return ret;
  }
}

fn unwrap_response<R: CandidType + DeserializeOwned>(
  response: Result<WasmResult, UserError>,
) -> R {
  match response.unwrap() {
    WasmResult::Reply(bytes) => candid::decode_one(&bytes).unwrap(),
    WasmResult::Reject(error) => panic!("{error}"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn test_query_transactions2() {
    let pic_env = PicEnv::new();
    // this is admin BTWL 
    let user1 = Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae").unwrap();

    let args = WalletAddCommand {
      address:
        "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
          .to_string(),
      principal_id: None, /* This is set to None as per your structure. You
                           * might need to set it as per your requirements. */
      from: "NNS".to_string(),
      name: "w1".to_string(),
    };

    let ret: Result<bool, String> =
      pic_env.my_update_call(user1, args, "add_wallet");

    match ret {
      Ok(_) => eprintln!("ok"),
      Err(_) => eprintln!("err"),
    }
  }
}
