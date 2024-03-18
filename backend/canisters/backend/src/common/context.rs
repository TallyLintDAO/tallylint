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

impl From<CanisterDB> for CanisterContext {
  /**
   * convert vec into bmap
   * TODO: maybe we can somehow direct store bmap into vec<u8> ?
   * todo this in order to decrease complexity and error prone.
   */
  fn from(payload: CanisterDB) -> Self {
    let users: BTreeMap<Principal, UserProfile> =
      payload.users.into_iter().map(|u| (u.owner, u)).collect();
    let wallets: BTreeMap<WalletId, WalletProfile> =
      payload.wallets.into_iter().map(|p| (p.id, p)).collect();

    let records: BTreeMap<RecordId, TransactionB> = payload
      .records
      .into_iter() //traverse each element in the Map instance
      //manipulate each element iterator gives
      .map(|v| (v.id, v))
      .collect();

    let transactions: BTreeMap<RecordId, TransactionF> = CONTEXT.with(|c| {
      let ctx = c.borrow_mut();
      ctx
        .trans_f_srv
        .transactions
        .clone()
        .into_iter()
        .map(|k| (k.0, k.1))
        .collect()
    });

    let neurons = payload
      .neurons
      .into_iter()
      .map(|p| (p.address.clone(), p))
      .collect();
    Self {
      id: payload.id,
      user_service: UserService { users },
      wallet_service: WalletService { wallets },
      wallet_transc_srv: WalletRecordService { records },
      neuron_service: NeuronService { neurons },
      trans_f_srv: TransactionService { transactions },
    }
  }
}

impl From<CanisterContext> for CanisterDB {
  fn from(context: CanisterContext) -> Self {
    let id = context.id;
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_transc_srv.records.values().cloned());
    let transactions =
      Vec::from_iter(context.trans_f_srv.transactions.values().cloned());
    let neurons =
      Vec::from_iter(context.neuron_service.neurons.values().cloned());
    Self {
      id,
      users,
      wallets,
      records,
      neurons,
      transactions,
    }
  }
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
