pub(crate) use candid::{CandidType, Principal};

use crate::{common::context::TimeStamp, transaction::domain::TransactionB};

use super::service::{RecordId, WalletAddress, WalletId};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct WalletUpdateCommand {
  //muttable
  pub from: String, /* from which wallet_type: such as
                     * NNS Plug  Stoic
                     * AstorMe  .. maybe add more */

  // muttable
  pub name: String,

  // immutable . for locate the wallet
  pub id: WalletId, /*when update , specify id .
                     * pub transactions:u64,
                     * pub last_sync_time:u64,
                     * pub last_transaction_time:u64, */
}

impl Default for WalletUpdateCommand {
  fn default() -> Self {
    WalletUpdateCommand {
      from: String::new(),
      name: String::new(),
      id: WalletId::default(), /* Assuming WalletId has a default
                                * implementation */
    }
  }
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
impl Default for WalletAddCommand {
  fn default() -> Self {
    WalletAddCommand {
      address: String::new(),
      principal_id: None,
      from: String::new(),
      name: String::new(),
    }
  }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct HistoryQueryCommand {
  // Primary key
  pub address: Vec<WalletAddress>, 
  pub from_time: TimeStamp,
  pub to_time: TimeStamp,
  // pub t_type: Option<String>, /* transaction_type SEND or
  //  * RECEIVE or BOTH */ //! front end do
  // pub tag: Option<Vec<String>>,   //! front end do 
  pub sort_method: Option<String>, /*by date-asc or date-desc
                                    * or profit-asc
                                    * profit-desc */
}
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct EditHistoryCommand {
  pub id: RecordId, //delete id here . dont need.
  pub coin_type: String,
  pub principal_id: Option<String>, /* Plug wallet need use , need
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
pub struct Wallet {
  walletid: u64,
  wallet_history: Vec<TransactionB>,
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
  hash: String,
  timestamp: u64, // TODO check ns or ms as unit
  t_type: String, //  transaction type : "SEND", "RECEIVE"
  walletName: String,
  details: Details,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Details {
  amount: f64,
  cost: f64, /* 由后端计算，理论上应该是不要持久化储存的，
              * 只有调用方法的时候由后端计算，组装 */
  currency: Currency,
  fee: f64,
  from: String,
  to: String,
  price: f64,
  value: f64,     //此笔交易价值
  status: String, //交易状态，表示交易成功与否，暂时先要着
  ledgerCanisterId: String,
  profit: f64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Currency {
  decimals: u64,  //代币精度
  symbol: String, //代币符号，例如'ICP'，'CHAT'
}
