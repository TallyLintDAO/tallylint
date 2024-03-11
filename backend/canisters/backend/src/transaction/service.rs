#[allow(unused_imports)]
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, HashMap};

use super::domain::*;

use crate::{common::context::TimeStamp, TransactionB};

#[allow(unused_imports)]
use crate::CONTEXT;

pub type WalletId = u64;
pub type TransactionId = u64;
pub type WalletAddress = String;
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct AddRecordCommand {
  pub coin_type: String,

  pub principal_id: Option<String>, /* Plug use , need
                                     * to convert to
                                     * opt_account_id_hex for use. */
  pub address: WalletAddress, // same as account_id_hex
  pub hash: String,
  pub t_type: String, //transaction_type
  pub status: String,
  pub time: TimeStamp, //transaction_time
  pub from: String,
  pub to: String,
  pub amount: u32,
  pub fee: f64,
  pub memo: String,
  pub price: f64,
  pub cost: f64,
  pub income: f64,
  pub profit: f64,
  pub tag: String,
  pub manual: bool,
  pub comment: String,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct EditHistoryCommand {
  pub coin_type: String,

  pub id: TransactionId, //delete id here . dont need.
  pub principal_id: Option<String>, /* Plug use , need
                          * to convert to
                          * opt_account_id_hex for use. */
  pub address: WalletAddress, // same as account_id_hex
  pub hash: String,
  pub t_type: String, //transaction_type
  pub status: String,
  pub time: TimeStamp, //transaction_time
  pub from: String,
  pub to: String,
  pub amount: u32,
  pub fee: f64,
  pub memo: String,
  pub price: f64,
  pub cost: f64,
  pub income: f64,
  pub profit: f64,
  pub tag: String,
  pub manual: bool,
  pub comment: String,
}

#[derive(Debug, Default)]
pub struct WalletRecordService {
  pub records: BTreeMap<TransactionId, TransactionB>,
}
#[derive(Debug, Default)]
pub struct TransactionService {
  pub transactions: BTreeMap<TransactionId, TransactionF>,
}
impl TransactionService {
  // TODO
  pub fn add_transaction_record(
    &mut self,
    id: u64,
    profile: TransactionF,
  ) -> Result<bool, String> {
    if self.transactions.contains_key(&id) {
      return Err("transaction record already exsit".to_string());
    }

    self.transactions.insert(id, profile);

    if self.transactions.contains_key(&id) {
      return Ok(true);
    } else {
      return Err("Insert fail. may heap overflow".to_string());
    }
  }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct HistoryQueryCommand {
  // Primary key
  pub address: Option<WalletAddress>, /* make this optional. if not
                                       * provide.
                                       * then query all. */
  pub from_time: TimeStamp,
  pub to_time: TimeStamp,
  pub t_type: String, /* transaction_type SEND or
                       * RECEIVE or BOTH */
  pub tag: String,
  //    TODO sort method:
  pub sort_method: String, /*by date-asc or date-desc
                            * or profit-asc
                            * profit-desc */
}
impl WalletRecordService {
  // TODO
  pub fn add_transaction_impl(
    &mut self,
    profile: TransactionB,
  ) -> Result<bool, String> {
    let id = profile.id;
    self.records.insert(profile.id, profile);
    if self.records.contains_key(&id) {
      return Ok(true);
    } else {
      return Err("Insert fail. may heap overflow".to_string());
    }
  }
  pub fn delete_transaction_impl(
    &mut self,
    id: TransactionId,
  ) -> Result<bool, String> {
    if !self.records.contains_key(&id) {
      return Err("transaction record not exsit".to_string());
    }

    self.records.remove(&id);

    if !self.records.contains_key(&id) {
      return Ok(true);
    } else {
      return Err("remove fail. still exsit".to_string());
    }
  }
  // pub fn get_addr_from_id(&self, id: TransactionId) -> WalletAddress {
  //   self.records.get(&id).unwrap().address.clone()
  // }
  pub fn wallet_history(
    &self,
    cmd: HistoryQueryCommand,
  ) -> Result<HashMap<WalletAddress, Vec<TransactionB>>, String> {
    if cmd.address.is_some() {
      let res = self.query_one_wallet(cmd);
      return Ok(res);
    } else { //query all
       // let wallets=WalletService::query_wallet_array(self,caller());
       // from ctx or ?
    }
    return Err("nothing".to_string());
  }

  // TODO make sort method work.
  //
  pub fn query_one_wallet(
    &self,
    cmd: HistoryQueryCommand,
  ) -> HashMap<String, Vec<TransactionB>> {
    let addr = cmd.address.unwrap().clone();
    let records = self.query_a_wallet_transactions(addr.clone());
    if records.is_empty() {
      return HashMap::new();
    }
    let mut res = HashMap::new();
    res.insert(addr.clone(), records);
    res
  }

  pub fn query_a_wallet_transactions(
    &self,
    addr: WalletAddress,
  ) -> Vec<TransactionB> {
    let transac = self
      .records
      .values()
      .filter(|record| record.address == addr)
      .cloned()
      .collect();
    return transac;
  }
}
