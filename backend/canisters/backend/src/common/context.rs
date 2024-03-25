use crate::{
  transaction::{
    domain::{TransactionB, TransactionF},
    service::{TransactionService, WalletRecordService},
  },
  wallet::service::{RecordId, WalletId},
  CONTEXT,
};
use candid::{CandidType, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::nns::domain::NeuronProfile;
use crate::nns::service::NeuronService;
use crate::user::domain::UserProfile;
use crate::user::service::UserService;
use crate::wallet::domain::WalletProfile;
use crate::wallet::service::WalletService;
use serde::{Deserialize, Serialize};

pub type TimeStamp = u64;
// pub type NeuronId_t = u64;
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

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct CanisterDB {
  pub id: u64,
  pub users: Vec<UserProfile>,
  pub wallets: Vec<WalletProfile>,
  pub records: Vec<TransactionB>,
  pub neurons: Vec<NeuronProfile>,
  pub transactions: Vec<TransactionF>,
}

// impl Default for CanisterContext {
//   fn default() -> Self {
//     Self {
//       id: 10001,
//       user_service: UserService::default(),
//       wallet_service: WalletService::default(),
//       wallet_transc_srv: WalletRecordService::default(),
//       neuron_service: NeuronService::default(),
//       trans_f_srv: TransactionService::default(),
//     }
//   }
// }

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
