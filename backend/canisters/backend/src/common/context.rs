use crate::{
  transaction::service::{TransactionService, WalletRecordService},
  
};
use candid::{CandidType, Principal};

use crate::nns::service::NeuronService;

use crate::user::service::UserService;

use crate::wallet::service::WalletService;
use serde::{Deserialize, Serialize};

use crate::lifecycle::init::CONTEXT;

pub type TimeStamp = u64;

// #[serde(deny_unknown_fields)] // can consider add this will err handing
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct CanisterContext {
  pub id: u64,
  pub user_service: UserService,
  pub wallet_service: WalletService,
  pub wallet_transc_srv: WalletRecordService,
  pub neuron_service: NeuronService,
  pub trans_f_srv: TransactionService,
}

impl CanisterContext {
  pub fn new() -> Self {
    CanisterContext {
      id: 10001,
      user_service: UserService::new(),
      wallet_service: WalletService::new(),
      wallet_transc_srv: WalletRecordService::new(),
      neuron_service: NeuronService::new(),
      trans_f_srv: TransactionService::new(),
    }
  }
}

use std::num::ParseIntError;
#[allow(dead_code)]
fn integer_part(value: &str) -> Result<u64, ParseIntError> {
  let dot_pos = value.find(".").unwrap_or(value.len());
  value[..dot_pos].parse()
}

pub fn generate_id() -> u64 {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    ctx.id = ctx.id + 1;
    ctx.id
  })
}
pub fn get_caller() -> Principal {
  return ic_cdk::caller();
}

pub fn now() -> u64 {
  return ic_cdk::api::time();
}

#[cfg(test)]
mod tests {
  use super::CanisterContext;

  #[test]
  fn deserialize_with_unknown() {
    let json = r#"{
  "id": 10002,
  "user_service": {
    "users": {
      "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae": {
        "owner": "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
        "name": "",
        "create_time": 1711439153466491724
      }
    }
  },
  "wallet_service": { "wallets": {} },
  "wallet_transc_srv": { "records": {} },
  "neuron_service": { "neurons": {} },
  "trans_f_srv": { "transactions": {} },
  "time_service": {}
}"#;
    // ! time_service  is not in rust type exsiting .
    // ! default behaviour will just ignore unknown (in rust type no matches for
    // it). !can set to strict mode  #[serde(deny_unknown_fields)]
    // can self write warn code to check if got unkonwn field.

    let result: Result<CanisterContext, _> = serde_json::from_str(json);

    match result {
      Ok(canister_context) => {
        eprintln!("Deserialization successful: {:?}", canister_context)
      }
      Err(e) => eprintln!("1 st Deserialization error: {}", e),
    }
  }
  #[test]
  fn deserialize_with_lack_of_field_json() {
    let json = r#"{
  "id": 10002,
  "user_service": {
    "users": {
      "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae": {
        "owner": "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
        "name": "",
        "create_time": 1711439153466491724
      }
    }
  },
  "wallet_service": { "wallets": {} },
  "wallet_transc_srv": { "records": {} },
  "neuron_service": { "neurons": {} },
  "trans_f_srv": { "transactions": {} }
}"#;

    // {
    //   "id": 10002,
    //   "user_service": {
    //     "users": {
    //       "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae":
    // {         "owner":
    // "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
    //         "name": "",
    //         "create_time": 1711439153466491724
    //       }
    //     },
    //     "configs": {}  // ! on purpose missing this . deserialize to rust
    // !must have this annotation  for type or fields: #[serde(default =
    // "BTreeMap::new")] type need this field   },
    //   "wallet_service": { "wallets": {} },
    //   "wallet_transc_srv": { "records": {} },
    //   "neuron_service": { "neurons": {} },
    //   "trans_f_srv": { "transactions": {} }
    // }

    let result: Result<CanisterContext, _> = serde_json::from_str(json);

    match result {
      Ok(canister_context) => {
        eprintln!("Deserialization successful: {:?}", canister_context)
      }
      Err(e) => eprintln!("2 nd Deserialization error: {}", e),
    }
  }
}
