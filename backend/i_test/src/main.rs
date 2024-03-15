use std::{collections::HashMap, fs::read};
use client::setup::CanisterId;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub mod backend_test;
pub mod client;

fn main() {
  test_query_transactions();
}

use candid::{encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, UserError, WasmResult};
use rand::random;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::client::setup::TERA;

fn test_query_transactions() {
  // init
  let pic_env = PicEnv::new();
  let user1 = Principal::from_text(
    // this is admin BTWL
    "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
  )
  .unwrap();

  // !register
  let reply: Result<UserProfile, String> =
    pic_env.my_update_call_no_arg(user1, "auto_register_user");
  match reply {
    Ok(data) => println!("{:?}", data),
    Err(_) => println!("err"),
  }

  // !add_wallet
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

  // !query wallet info
  let ret: Result<Vec<WalletProfile>, Vec<WalletProfile>> =
    pic_env.my_query_call_no_arg(user1, "query_all_wallets");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(_) => println!("err"),
  }

  // !add transactions
  let transaction = TransactionF {
    hash: "123".to_string(),
    timestamp: 123.0,
    t_type: "SEND".to_string(),
    walletName: "asd".to_string(),
    details: Details {
      amount: 123.8,
      cost: 1.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      from: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739".to_string(),
      to: "asd".to_string(),
      price: 1.0,
      value: 1.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 1.0,
    },
  };
  let sync_transaction_command = SyncTransactionCommand {
    // todo this id should get from last op of `query_all_wallets`
    walletId: 10002,
    history: vec![transaction],
  };
  let args: Vec<SyncTransactionCommand> = vec![sync_transaction_command];
  let ret: Result<bool, String> =
    pic_env.my_update_call(user1, args, "sync_transaction_record");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }

  // !query transactions
  // generate_query_call!(query_wallet_transactions);
  let args: HistoryQueryCommand = HistoryQueryCommand {
    address: (vec![
      "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
    ]),
    from_time: 0, // Replace with your actual timestamp
    to_time: 0,   // Replace with your actual timestamp
    sort_method: Some("date-asc".to_string()),
  };
  let ret: Result<HashMap<WalletAddress, Vec<TransactionB>>, String> =
    pic_env.my_query_call(user1, args, "query_wallet_transactions");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }

  // !query payload DB
  let ret: String =
    pic_env.my_query_call_no_arg(user1, "collect_running_payload");
  println!("{:?}", ret);
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
use termion::color;

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
    print_red("####Executing:  ".to_owned() + method);
    let ret_raw = self.pic.update_call(
      self.canister_id,
      user,
      method,
      candid::encode_one(args).unwrap(),
    );

    let ret: ResponseType = unwrap_response(ret_raw);
    return ret;
  }

  fn my_update_call_no_arg<
    ResponseType: candid::CandidType + DeserializeOwned,
  >(
    &self,
    user: Principal,
    method: &str,
  ) -> ResponseType {
    print_red("####Executing:  ".to_owned() + method);

    let ret_raw = self.pic.update_call(
      self.canister_id,
      user,
      method,
      candid::encode_one(()).unwrap(),
    );

    let ret: ResponseType = unwrap_response(ret_raw);
    return ret;
  }

  fn my_query_call<
    ArgsType: candid::CandidType,
    ResponseType: candid::CandidType + DeserializeOwned,
  >(
    &self,
    user: Principal,
    args: ArgsType,
    method: &str,
  ) -> ResponseType {
    print_red("####Executing:  ".to_owned() + method);

    let ret_raw = self.pic.query_call(
      self.canister_id,
      user,
      method,
      candid::encode_one(args).unwrap(),
    );

    let ret: ResponseType = unwrap_response(ret_raw);
    return ret;
  }
  fn my_query_call_no_arg<
    ResponseType: candid::CandidType + DeserializeOwned,
  >(
    &self,
    user: Principal,
    method: &str,
  ) -> ResponseType {
    print_red("####Executing:  ".to_owned() + method);

    let ret_raw = self.pic.query_call(
      self.canister_id,
      user,
      method,
      encode_one(()).unwrap(),
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
    let user1 = Principal::from_text(
      "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
    )
    .unwrap();

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
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct UserProfile {
  pub owner: Principal, // 用户 Principal
  pub name: String,
  pub create_time: u64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Wallet {
  walletid: u64,
  wallet_history: Vec<TransactionF>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct SyncTransactionCommand {
  pub walletId: WalletId,
  pub history: Vec<TransactionF>,
}

/**
 * FIXED DATA TYPE, use by frontend. dont change easily.
 *
 * B stands for backend data
 * F stands for frontend data type
 *
 */
#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionF {
  pub hash: String,
  pub timestamp: f64, // TODO check ns or ms as unit
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub walletName: String,
  pub details: Details,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Details {
  pub amount: f64,
  pub cost: f64, /* 由后端计算，理论上应该是不要持久化储存的，
                  * 只有调用方法的时候由后端计算，组装 */
  pub currency: Currency,
  pub fee: f64,
  pub from: String,
  pub to: String,
  pub price: f64,
  pub value: f64,     //此笔交易价值
  pub status: String, //交易状态，表示交易成功与否，暂时先要着
  pub ledgerCanisterId: String,
  pub profit: f64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Currency {
  pub decimals: u64,  //代币精度
  pub symbol: String, //代币符号，例如'ICP'，'CHAT'
}
pub type WalletId = u64;
pub type TransactionId = u64;
pub type WalletAddress = String;

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionB {
  //
  // backend autogen:
  pub id: TransactionId,
  //
  pub hash: String,
  pub timestamp: f64, //this is ms format with float.
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub walletName: String,
  pub details: Details,

  pub principal_id: Option<String>, /* Plug use , need
                                     * to convert to
                                     * opt_account_id_hex for use. */
  pub memo: String,
  pub address: WalletAddress,

  pub tag: String,
  pub manual: bool,
  pub comment: String,
  // TODO , considering wallet_amount :
  // pub wallet_amount:u32,
  // pub warning:String,
  // TODO: Warning（用户是否标记某些记录为missing cost,
  // missing rates）这条字段先只做出来，不用,
  // 解释：比如missing
  // rates是标记某个交易历史找不到对应的价格记录，
  // 例如某个NFT的交易价格查不到，
  // 就会被自动标记为missing rates
}

fn print_red(s: String) {
  println!("{}{}{}", color::Fg(color::Red), s, color::Fg(color::Reset));
}

pub type TimeStamp = u64;
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct HistoryQueryCommand {
  // Primary key
  pub address: Vec<WalletAddress>, /* make this optional. if not
                                    * provide.
                                    * then query all. */
  pub from_time: TimeStamp,
  pub to_time: TimeStamp,
  // pub t_type: Option<String>, /* transaction_type SEND or
  //  * RECEIVE or BOTH */
  // pub tag: Option<Vec<String>>,
  pub sort_method: Option<String>, /*by date-asc or date-desc
                                    * or profit-asc
                                    * profit-desc */
}
