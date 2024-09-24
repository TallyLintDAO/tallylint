#[allow(unused_imports)]
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, HashMap};

use super::domain::*;

use crate::common::times::{
  timestamp_ms_float_to_ms_u64, timestamp_ms_float_to_ns,
};
use crate::transaction::domain::SyncTransactionCommand;
use crate::wallet::domain::HistoryQueryCommand;
use crate::{
  common::context::TimeStamp, wallet::service::WalletId, TransactionB,
};
//FIXME 垃圾，数据恢复后将其删除
pub type WalletAddress = String;
//FIXME 该数据结构需要删除
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
//FIXME 该数据结构需要删除
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

//该交易记录用于接收前端的数据并进行临时的存储和删除
//FIXME 整个TransactionService都是垃圾，全部删除
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionService {
  pub transactions: BTreeMap<WalletId, TransactionF>,
}
impl TransactionService {
  // TODO
  /**
   * insert the transaction record
   */
  pub fn add_transaction_record(
    &mut self,
    id: u64,
    record: TransactionF,
  ) -> Result<bool, String> {
    // check if there is a transaction with the same wid and hash
    for (_, existing_record) in &self.transactions {
      if existing_record.wid == record.wid
        && existing_record.hash == record.hash
      {
        return Err(
          "Transaction with the same wid and hash already exists".to_string(),
        );
      }
    }

    // add the record
    self.transactions.insert(id, record);

    // check if it is added successfully
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
  /**
   * 根据钱包地址进行删除
   */
  pub fn delete_all_by_addr(
    &mut self,
    addr: String,
    principal_id: Option<String>,
  ) -> bool {
    let keys_to_remove: Vec<u64> = self
      .transactions
      .iter()
      .filter(|(_, trans_f)| {
        // 检查交易是否匹配 addr
        let addr_match = (trans_f.t_type == "SEND"
          && trans_f.details.from == addr)
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
  /**
   * 根据walletId进行删除
   */
  pub fn delete_all_by_wid(&mut self, id: WalletId) -> bool {
    let keys_to_remove: Vec<u64> = self
      .transactions
      .iter()
      .filter(|(_, trans_f)| trans_f.wid == id)
      .map(|(key, _)| *key)
      .collect();

    //if didn't find out keys to remove, return error
    if keys_to_remove.is_empty() {
      return false;
    }

    for key in keys_to_remove {
      self.transactions.remove(&key);
    }

    true
  }
}

//这个service主要是用于处理同步过后的交易记录
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct WalletRecordService {
  pub records: BTreeMap<TransactionId, TransactionB>,
  #[serde(default = "BTreeMap::new")]
  pub my_summary: BTreeMap<u64, MySummary>,
}

impl WalletRecordService {
  //insert transaction
  pub fn add_transaction_impl(
    &mut self,
    record: TransactionB,
    ctx_id: u64,
  ) -> Result<u64, String> {
    let id = ctx_id + 1;
    // check if there is a transaction with the same wid and hash
    for (_, existing_record) in &self.records {
      if existing_record.wid == record.wid
        && existing_record.hash == record.hash
      {
        return Err(
          "Transaction with the same wid and hash already exists".to_string(),
        );
      }
    }

    // add the record
    self.records.insert(id, record);

    // check if it is added successfully
    if self.records.contains_key(&id) {
      return Ok(id);
    } else {
      return Err("Insert failed. Possible heap overflow".to_string());
    }
  }
  pub fn new() -> Self {
    WalletRecordService {
      records: BTreeMap::new(),
      my_summary: BTreeMap::new(),
    }
  }
  //修改交易记录
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
  //单个查询交易记录
  pub fn query_one(&mut self, id: WalletId) -> Result<TransactionB, String> {
    match self.records.get(&id) {
      Some(transaction) => Ok(transaction.clone()),
      None => Err(format!("No transaction found with id: {}", id)),
    }
  }
  //根据钱包地址进行交易记录的删除
  pub fn delete_transaction_by_addr(&mut self, addr: &WalletAddress) {
    self
      .records
      .retain(|_index, transaction| transaction.address != *addr);
  }
  //根据用户principal进行删除
  pub fn delete_transaction_by_principal(&mut self, principal_id: &String) {
    self
      .records
      .retain(|_index, transaction| transaction.address != *principal_id);
  }
  //根据钱包id进行交易记录的删除
  pub fn delete_transactions_by_wid(
    &mut self,
    wid: WalletId,
  ) -> Result<bool, String> {
    // Debug log for tracking
    println!("Deleting transaction for wid: {:?}", wid);

    // Find the TransactionId that corresponds to the given WalletId
    let transaction_id = self.records.iter().find_map(|(id, transaction)| {
      if transaction.wid == wid {
        Some(*id)
      } else {
        None
      }
    });

    match transaction_id {
      Some(id) => {
        self.records.remove(&id);
        // Verify if removal was successful
        if self.records.contains_key(&id) {
          Err("Remove failed. Record still exists.".to_string())
        } else {
          Ok(true)
        }
      }
      None => Err(format!(
        "Transaction record for wid {:?} does not exist",
        wid
      )),
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
  pub fn query_synced_transactions(
    &self,
    cmd: HistoryQueryCommand,
  ) -> Vec<TransactionB> {
    let mut sync_transactions: Vec<TransactionB> = cmd
      .wids
      .iter()
      .flat_map(|wid| {
        self
          .query_one_wallet_trans_by_wallet_id(wid.clone())
          .get(wid)
          .cloned()
          .unwrap_or_default()
      })
      .collect();

    // Filter transactions by time range if specified
    if cmd.from_time != 0 && cmd.to_time != 0 {
      sync_transactions.retain(|transaction| {
        transaction.timestamp >= cmd.from_time
          && transaction.timestamp <= cmd.to_time
      });
    }

    if sync_transactions.is_empty() {
      return sync_transactions;
    }

    // Sort transactions based on the provided sort method
    if let Some(method) = cmd.sort_method {
      match method.as_str() {
        "date-asc" => sync_transactions.sort_by_key(|t| t.timestamp),
        "date-desc" => {
          sync_transactions.sort_by_key(|t| std::cmp::Reverse(t.timestamp))
        }
        "profit-asc" => sync_transactions.sort_by(|a, b| {
          a.details
            .profit
            .partial_cmp(&b.details.profit)
            .unwrap_or(std::cmp::Ordering::Equal)
        }),
        "profit-desc" => sync_transactions.sort_by(|a, b| {
          b.details
            .profit
            .partial_cmp(&a.details.profit)
            .unwrap_or(std::cmp::Ordering::Equal)
        }),
        _ => sync_transactions.sort_by_key(|t| t.timestamp), // Default sort by date-asc
      }
    } else {
      sync_transactions.sort_by_key(|t| t.timestamp); // Default sort if no method is provided
    }

    sync_transactions
  }
  //通过钱包地址查询单个钱包的交易记录
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
  //通过wid查询单个钱包交易记录
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
  //查询所有钱包的交易记录
  pub fn query_all_transactions(&self) -> HashMap<WalletId, TransactionB> {
    let mut all_trans = HashMap::new();
    for (id, records) in &self.records {
      all_trans.insert(id.clone(), records.clone());
    }
    return all_trans;
  }
}
