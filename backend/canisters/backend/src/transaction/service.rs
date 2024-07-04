#[allow(unused_imports)]
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, HashMap};

use super::domain::*;

use crate::{
  common::context::TimeStamp, wallet::service::WalletId, TransactionB,
};

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

  pub id: WalletId, //delete id here . dont need.
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
pub struct WalletRecordService {
  pub records: BTreeMap<TransactionId, TransactionB>,
  #[serde(default = "BTreeMap::new")]
  pub my_summary: BTreeMap<u64, MySummary>,
}
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionService {
  pub transactions: BTreeMap<WalletId, TransactionF>,
}
impl TransactionService {
  // TODO
  pub fn add_transaction_record(
    &mut self,
    id: u64,
    profile: TransactionF,
) -> Result<bool, String> {
    if self.transactions.contains_key(&id) {
        return Err("Transaction record already exists".to_string());
    }

    self.transactions.insert(id, profile);

    // 检查插入是否成功
    if self.transactions.contains_key(&id) {
        return Ok(true);
    } else {
        return Err("Insert failed. Possible heap overflow".to_string());
    }
}

  pub fn new() -> Self {
    TransactionService {
      transactions: BTreeMap::new(),
    }
  }

  pub fn contains(&mut self, id: WalletId) -> bool {
    if self.transactions.contains_key(&id) {
      return true;
    }
    return false;
  }

  pub fn delete_all_by_addr(&mut self, addr: String, principal_id: Option<String>) -> bool {
    let keys_to_remove: Vec<u64> = self
      .transactions
      .iter()
      .filter(|(_, trans_f)| {
        // 检查交易是否匹配 addr
        let addr_match = (trans_f.t_type == "SEND" && trans_f.details.from == addr)
            || trans_f.details.to == addr;

        // 检查交易是否匹配 principal_id
        let principal_match = if let Some(ref pid) = principal_id {
            (trans_f.t_type == "SEND" && trans_f.details.from == *pid)
                || trans_f.details.to == *pid
        } else {
            false
        };

        addr_match || principal_match
    })
      .map(|(key, _)| *key)
      .collect();

    for key in keys_to_remove {
      self.transactions.remove(&key);
    }

    true
  }

  pub fn delete_all_by_wid(&mut self, id: WalletId) -> bool {
    let keys_to_remove: Vec<u64> = self
      .transactions
      .iter()
      .filter(|(_, trans_f)| trans_f.wid == id)
      .map(|(key, _)| *key)
      .collect();

    for key in keys_to_remove {
      self.transactions.remove(&key);
    }

    true
  }
}

impl WalletRecordService {
  pub fn add_transaction_impl(
    &mut self,
    profile: TransactionB,
  ) -> Result<bool, String> {
    let id =profile.id;
    self.records.insert(id, profile);
    if self.records.contains_key(&id) {
      return Ok(true);
    } else {
      return Err("Insert fail. may heap overflow".to_string());
    }
  }
  pub fn new() -> Self {
    WalletRecordService {
      records: BTreeMap::new(),
      my_summary: BTreeMap::new(),
    }
  }
  pub fn update_transaction_impl(
    &mut self,
    profile: TransactionB,
  ) -> Result<bool, String> {
    let id = profile.id;
    self.records.insert(profile.id, profile);
    if self.records.contains_key(&id) {
      return Ok(true);
    } else {
      return Err("Update fail. may heap overflow".to_string());
    }
  }

  pub fn query_one(&mut self, id: WalletId) -> Result<TransactionB, String> {
    match self.records.get(&id) {
      Some(transaction) => Ok(transaction.clone()),
      None => Err(format!("No transaction found with id: {}", id)),
    }
  }

  pub fn delete_transaction_by_addr(&mut self, addr: &WalletAddress) {
    self
      .records
      .retain(|_index, transaction| transaction.address != *addr);
  }
  pub fn delete_transaction_by_principal(&mut self, principal_id: &String) {
    self
      .records
      .retain(|_index, transaction| transaction.address != *principal_id);
  }

  pub fn delete_transaction_by_id_impl(
    &mut self,
    id: WalletId,
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
  // pub fn wallet_history(
  //   &self,
  //   cmd: HistoryQueryCommand,
  // ) -> Result<HashMap<WalletAddress, Vec<TransactionB>>, String> {
  //   // if cmd.address.is_some() {
  //   //   let res = self.query_one_wallet(cmd);
  //   //   return Ok(res);
  //   // } else { //query all
  //   //    // let wallets=WalletService::query_wallet_array(self,caller());
  //   //    // from ctx or ?
  //   // }
  //   return Err("nothing".to_string());
  // }

  pub fn query_one_wallet_trans(
    &self,
    addr: WalletAddress,
  ) -> HashMap<WalletAddress, Vec<TransactionB>> {
    let mut one_wallet = HashMap::new();
    let records: Vec<TransactionB> = self
      .records
      .values()
      .filter(|record| record.address == addr)
      .cloned()
      .collect();
    if records.is_empty() {
      return HashMap::new();
    }
    one_wallet.insert(addr.clone(), records);
    return one_wallet;
  }

  pub fn query_one_wallet_trans_by_wallet_id(
    &self,
    wid: WalletId,
  ) -> HashMap<WalletId, Vec<TransactionB>> {
    let mut one_wallet = HashMap::new();
    let records: Vec<TransactionB> = self
      .records
      .values()
      .filter(|record| record.wid == wid)
      .cloned()
      .collect();
    if records.is_empty() {
      return HashMap::new();
    }
    one_wallet.insert(wid.clone(), records);
    return one_wallet;
  }

  pub fn query_all_transactions(&self) -> HashMap<WalletId, TransactionB> {
    let mut all_trans = HashMap::new();
    for (id, records) in &self.records {
      all_trans.insert(id.clone(), records.clone());
    }
    return all_trans;
  }
}
