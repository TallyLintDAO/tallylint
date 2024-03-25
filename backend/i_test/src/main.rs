use client::{setup::CanisterId, unwrap_response};
use std::{
  collections::HashMap,
  fs::{read, File},
  io::Read,
};
pub mod backend_test;
pub mod client;

fn main() {
  // TODO exec this before test:
  // cd git root
  // cargo build --release --target wasm32-unknown-unknown --package backend &&
  // candid-extractor target/wasm32-unknown-unknown/release/backend.wasm
  // >./backend/canisters/backend/backend.did

  test_crud_transactions();
  // test_db_update();
}

fn test_db_update() {
  let (pic_env, user_admin) = init();
  user_register(&pic_env, user_admin);
  let terminal_args: Vec<String> = std::env::args().collect();

  // get_payload_from_dropbox(&pic_env, user_admin, terminal_args[1].clone());
  // get_payload_from_my_server(&pic_env, user_admin);
  send_payload_string_to_canister(&pic_env, user_admin);
}

use candid::{encode_one, CandidType, Principal};
use pocket_ic::{PocketIc, UserError, WasmResult};
use rand::random;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::client::{rng::random_principal, setup::TERA};

fn test_crud_transactions() {
  // !init
  let (pic_env, user_admin) = init();

  // !register
  user_register(&pic_env, user_admin);
  // !add_wallet
  add_wallet(&pic_env, user_admin);

  // !query wallet info
  query_all_wallet_info(&pic_env, user_admin);

  // !add transactions
  sync_transactions_from_front_end(&pic_env, user_admin);
  // !query payload DB
  // query_payload_db(&pic_env, user_admin);

  // !calculate_tax
  calculate_tax(&pic_env, user_admin);

  // !simple query transactions
  no_filter_no_sort_simple_transac_query(&pic_env, user_admin);

  // !time range query test
  time_range_test(&pic_env, user_admin);
  // !sort method query test
  sort_method_test(&pic_env, user_admin);

  // !edit transacitons:
  // add a new tranc as mannual flag.then add some tag, add a comment
  add_a_completx_transaction(&pic_env, user_admin);
  // tag as 3333 keyword serach if ok
  update_completx_transaction(&pic_env, user_admin);
  query_a_completx_transaction(&pic_env, user_admin);
  // !simple query wallet-linked-transactions
  no_filter_no_sort_simple_transac_query(&pic_env, user_admin);

  // !query wallet info
  query_all_wallet_info(&pic_env, user_admin);
}

fn tax_calculation(pic_env: &PicEnv, user1: Principal) {

  let ret: Result<bool, String> =
    pic_env.my_update_call(user1, args, "calculate_tax");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }

}

fn init() -> (PicEnv, Principal) {
  let pic_env = PicEnv::new();
  let user_admin = Principal::from_text(
    // TODO this caller is clear inputed with un encrtpted. can hacker using
    // this as caller ? this is admin BTWL
    "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
  )
  .unwrap();
  // let user2 = random_principal();
  // // !register
  // let reply: Result<UserProfile, String> =
  //   pic_env.my_update_call_no_arg(user2, "auto_register_user");
  // match reply {
  //   Ok(data) => println!("{:?}", data),
  //   Err(_) => println!("err"),
  // }
  (pic_env, user_admin)
}

fn user_register(pic_env: &PicEnv, user_admin: Principal) {
  let reply: Result<UserProfile, String> =
    pic_env.my_update_call_no_arg(user_admin, "auto_register_user");
  match reply {
    Ok(data) => println!("{:?}", data),
    Err(_) => println!("err"),
  }
}
fn get_payload_from_dropbox(
  pic_env: &PicEnv,
  user_admin: Principal,
  token: String,
) {
  // 1u64 is a literal that represents an unsigned 64-bit integer with a value
  // of 1
  let args = candid::encode_args((token, "02")).unwrap();
  let reply: String = pic_env.my_update_call_many_args(
    user_admin,
    args,
    "get_payload_from_dropbox",
  );
  println!("{:?}", reply);
}

fn get_payload_from_my_server(pic_env: &PicEnv, user_admin: Principal) {
  let reply: String =
    pic_env.my_update_call_no_arg(user_admin, "get_payload_from_my_server");
  println!("{:?}", reply);
}

fn send_payload_string_to_canister(pic_env: &PicEnv, user_admin: Principal) {
  let data=read_db_to_string_from_local_json_file("/home/btwl/code/ic/tax_lint/backend/i_test/new_ctx_struct_all_ic_data.json".to_owned());
  let reply: String =
    pic_env.my_update_call(user_admin, data, "send_payload_string_to_canister");
  println!("{:?}", reply);
}

fn add_wallet(pic_env: &PicEnv, user_admin: Principal) {
  let args = WalletAddCommand {
    address: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
      .to_string(),
    principal_id: None, /* This is set to None as per your structure. You
                         * might need to set it as per your requirements. */
    from: "NNS".to_string(),
    name: "w1".to_string(),
  };
  let ret: Result<bool, String> =
    pic_env.my_update_call(user_admin, args, "add_wallet");
  match ret {
    Ok(_) => println!("ok"),
    Err(_) => println!("err"),
  }
}

fn query_all_wallet_info(pic_env: &PicEnv, user_admin: Principal) {
  let ret: Result<Vec<WalletProfile>, Vec<WalletProfile>> =
    pic_env.my_query_call_no_arg(user_admin, "query_all_wallets");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(_) => println!("err"),
  }
}

fn sync_transactions_from_front_end(pic_env: &PicEnv, user1: Principal) {
  let transaction1 = TransactionF {
    hash: "123".to_string(),
    timestamp: 10.0,
    t_type: "RECEIVE".to_string(),
    details: Details {
      amount: 10.0,
      cost: 50.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 0.8,
      from: "asd".to_string(),
      to: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      price: 5.0,
      value: 5.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };
  let transaction2 = TransactionF {
    hash: "12z33".to_string(),
    timestamp: 210.0,
    t_type: "SEND".to_string(),

    details: Details {
      amount: 5.0,
      cost: 40.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      from: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      to: "asd".to_string(),
      price: 8.0,
      value: 8.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };
  let transaction3 = TransactionF {
    hash: "123".to_string(),
    timestamp: 211.0,
    t_type: "RECEIVE".to_string(),

    details: Details {
      amount: 3.0,
      cost: 60.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      to: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      from: "asd".to_string(),
      price: 20.0,
      value: 20.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };
  let transaction4 = TransactionF {
    hash: "123".to_string(),
    timestamp: 222.0,
    t_type: "SEND".to_string(),

    details: Details {
      amount: 2.0,
      cost: 8.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      from: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      to: "asd".to_string(),
      price: 4.0,
      value: 4.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };
  let transaction5 = TransactionF {
    hash: "123".to_string(),
    timestamp: 3333.0,
    t_type: "RECEIVE".to_string(),

    details: Details {
      amount: 2.0,
      cost: 12.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      to: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      from: "asd".to_string(),
      price: 6.0,
      value: 6.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };
  let transaction6 = TransactionF {
    hash: "123".to_string(),
    timestamp: 1000.0,
    t_type: "SEND".to_string(),

    details: Details {
      amount: 8.0,
      cost: 112.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      to: "asd".to_string(),
      from: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      price: 14.0,
      value: 14.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };
  let sync_transaction_command = SyncTransactionCommand {
    // todo this id should get from last op of `query_all_wallets`
    walletId: 10002,
    history: vec![
      transaction1,
      transaction2,
      transaction3,
      transaction4,
      transaction5,
      transaction6,
    ],
  };
  let args: Vec<SyncTransactionCommand> = vec![sync_transaction_command];
  let ret: Result<bool, String> =
    pic_env.my_update_call(user1, args, "sync_transaction_record");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }
}

// add a new tranc as mannual flag.then add some tag, add a comment
fn add_a_completx_transaction(pic_env: &PicEnv, user1: Principal) {
  let trans_f = TransactionF {
    hash: "123".to_string(),
    timestamp: 10.0,
    t_type: "SEND".to_string(),

    details: Details {
      amount: 123.8,
      cost: 1.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      from: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      to: "asd".to_string(),
      price: 1.0,
      value: 1.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };

  let mut args: TransactionB = convert_trans_f_to_trans_b(trans_f, 110050);
  args.manual = true;
  args.comment = "my manualled blabala".to_string();
  args.tag.push("air drop".to_string());
  args.tag.push("air drop2222".to_string());
  let ret: Result<TransactionId, String> =
    pic_env.my_update_call(user1, args, "add_transaction");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }
}
// TODO
fn update_completx_transaction(pic_env: &PicEnv, user1: Principal) {
  let trans_f = TransactionF {
    hash: "123".to_string(),
    timestamp: 10.0,
    t_type: "SEND".to_string(),

    details: Details {
      amount: 123.8,
      cost: 1.0,
      currency: Currency {
        decimals: 2,
        symbol: "ICP".to_string(),
      },
      fee: 123.8,
      from: "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
      to: "asd".to_string(),
      price: 1.0,
      value: 1.0,
      status: "SUCCESS".to_string(),
      ledgerCanisterId: "asd".to_string(),
      profit: 0.0,
    },
  };

  let mut args: TransactionB = convert_trans_f_to_trans_b(trans_f, 10016);

  args.manual = true;
  args.comment = "my manualled blabala".to_string();
  args.tag.push("air drop".to_string());
  args.tag.push("air drop3333".to_string());

  let ret: Result<bool, String> =
    pic_env.my_update_call(user1, args, "update_transaction");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }
}

// TODO
fn query_a_completx_transaction(pic_env: &PicEnv, user1: Principal) {
  let ret: Result<TransactionB, String> =
    pic_env.my_update_call(user1, 10016 as u64, "query_one_transaction");
  match ret {
    Ok(data) => println!("{:?}", data),
    Err(err) => println!("{:?}", err),
  }
}

fn query_payload_db(pic_env: &PicEnv, user1: Principal) {
  let ret: String =
    pic_env.my_query_call_no_arg(user1, "collect_running_payload");
  println!("{:?}", ret);
}

fn no_filter_no_sort_simple_transac_query(pic_env: &PicEnv, user1: Principal) {
  let args: HistoryQueryCommand = HistoryQueryCommand {
    address: (vec![
      "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
    ]),
    from_time: 0, // Replace with your actual timestamp
    to_time: 0,   // Replace with your actual timestamp
    sort_method: None,
  };
  let ret: Vec<SimpleTransaction> =
    pic_env.my_query_call(user1, args, "query_wallets_synced_transactions");
  println!("{:?}", ret);
}
fn calculate_tax(pic_env: &PicEnv, user1: Principal) {
  let args = candid::encode_args((
    vec![
      "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
    ],
    "fifo".to_string(),
    vec!["none".to_string()],
  ))
  .unwrap();
  let ret: String =
    pic_env.my_update_call_many_args(user1, args, "calculate_tax");
  println!("{:?}", ret);
}

fn sort_method_test(pic_env: &PicEnv, user1: Principal) {
  let args_sort_method: HistoryQueryCommand = HistoryQueryCommand {
    address: vec![
      "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
    ],
    from_time: 0,
    to_time: 0,
    sort_method: Some("date-desc".to_string()), /* Change to your desired
                                                 * sort method */
  };
  let ret_sort_method: Vec<SimpleTransaction> = pic_env.my_query_call(
    user1,
    args_sort_method,
    "query_wallets_synced_transactions",
  );
  print_red_header(
    "====date-desc Sort method query result: ".to_string(),
    format!("{:?}", ret_sort_method),
  );

  let args = HistoryQueryCommand {
    address: vec![
      "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
    ],
    from_time: 0,
    to_time: 0,
    sort_method: Some("profit-desc".to_string()), /* Change to your desired
                                                   * sort method */
  };
  let res: Vec<SimpleTransaction>=
  // !This is a kind of RPC . should have good reflection way of handle input type and output type 
    pic_env.my_query_call(user1, args, "query_wallets_synced_transactions");
  print_red_header(
    "====profit-desc Sort method query result: ".to_string(),
    format!("{:?}", res),
  );
}

fn time_range_test(pic_env: &PicEnv, user1: Principal) {
  let args_time_range: HistoryQueryCommand = HistoryQueryCommand {
    address: vec![
      "307b116d3afaebde45e59b1cf4ec717f30059c10eeb5f8e93d3316d2562cf739"
        .to_string(),
    ],
    from_time: 100_000_000, // ns as u64   ms to ns append  6 zero
    to_time: 500_000_000,
    sort_method: None,
  };
  let ret_time_range: Vec<SimpleTransaction> = pic_env.my_query_call(
    user1,
    args_time_range,
    "query_wallets_synced_transactions",
  );
  println!(" should be 3 result . full 6 result");
  print_red_header(
    "====Time range query result: ".to_string(),
    format!("{:?}", ret_time_range),
  );
}

fn print_red_header(red_string: String, origin_color_string: String) {
  println!(
    "{}{}{}{}",
    color::Fg(color::Red),
    red_string,
    color::Fg(color::Reset),
    origin_color_string
  );
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

  fn my_update_call_many_args<
    ResponseType: candid::CandidType + DeserializeOwned,
  >(
    &self,
    user: Principal,
    args: Vec<u8>,
    method: &str,
  ) -> ResponseType {
    print_red("####Executing:  ".to_owned() + method);
    let ret_raw = self.pic.update_call(self.canister_id, user, method, args);

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

#[cfg(test)]
mod tests {
  use super::*;

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
  // pub walletName: String,
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
  pub decimals: u8,   //代币精度
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
  pub timestamp: u64, //this is ms format with float.
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub details: Details,

  pub memo: String,
  pub address: WalletAddress,

  pub tag: Vec<String>,
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

fn convert_trans_f_to_trans_b(
  trans_f: TransactionF,
  id: TransactionId,
) -> TransactionB {
  let address = match trans_f.t_type.as_str() {
    "SEND" => trans_f.details.from.clone(),
    "RECEIVE" => trans_f.details.to.clone(),
    _ => WalletAddress::default(), // You can handle other cases here
  };

  TransactionB {
    id,
    hash: trans_f.hash,
    timestamp: timestamp_ms_float_to_ns(trans_f.timestamp),
    t_type: trans_f.t_type,
    details: trans_f.details.clone(),
    memo: String::new(),
    address,
    tag: Vec::new(),
    manual: false,
    comment: String::new(),
  }
}

// Function to convert milliseconds to nanoseconds
pub fn timestamp_ms_float_to_ns(milliseconds: f64) -> u64 {
  (milliseconds * 1_000_000.0) as u64
}

fn read_db_to_string_from_local_json_file(f_path: String) -> String {
  let mut file = File::open(f_path).expect("Unable to open the file");
  let mut db_json = String::new();
  file
    .read_to_string(&mut db_json)
    .expect("Unable to read the file");
  db_json
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct SimpleTransaction {
  //
  // backend autogen:
  pub id: TransactionId,
  //
  pub hash: String,
  pub timestamp: u64, //this is ns format usigned 64bit
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub details: Details,
  pub tag: Vec<String>,
  pub manual: bool, // if this trasac is manual import
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
