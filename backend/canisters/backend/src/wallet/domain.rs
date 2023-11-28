pub(crate) use candid::{CandidType, Deserialize, Principal};

use crate::common::context::TimeStamp;

use super::service::{RecordId, WalletAddress, WalletId};

#[derive(Debug, Clone, CandidType, Deserialize)]
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

#[derive(Debug, Clone, CandidType, Deserialize)]
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
#[derive(Debug, Clone, CandidType, Deserialize)]
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

/**
 * Class Record main storage:
 */
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct RecordProfile {
  pub id: RecordId,
  // Primary key
  pub principal_id: Option<String>, /* Plug use , need
                                     * to convert to
                                     * opt_account_id_hex for use. */
  pub address: WalletAddress, // same as account_id_hex
  //Transaction record
  pub price: f64,
  pub amount: u32,
  // TODO , considering wallet_amount :
  // pub wallet_amount:u32,
  pub time: TimeStamp, //transaction_time
  pub t_type: String,  //transaction_type
  pub tag: String,
  pub manual: bool,
  pub comment: String,
  // pub warning:String,
  // TODO: Warning（用户是否标记某些记录为missing cost,
  // missing rates）这条字段先只做出来，不用,
  // 解释：比如missing
  // rates是标记某个交易历史找不到对应的价格记录，
  // 例如某个NFT的交易价格查不到，
  // 就会被自动标记为missing rates
}

#[derive(Debug, Clone, CandidType, Deserialize)]
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
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EditHistoryCommand {
  pub id: RecordId, //delete id here . dont need.
  //Transaction record
  pub price: f64,
  pub amount: u32,
  // TODO , considering:
  // pub wallet_amount:u32,
  pub time: TimeStamp, //transaction_time
  pub t_type: String,  //transaction_type
  pub tag: String,
  pub manual: bool,
  pub comment: String,
}
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct AddRecordCommand {
  // pub id: RecordId,//delete id here . dont need.
  //Transaction record
  pub address: WalletAddress,
  pub principal_id: Option<String>, /* Plug use , need
                                     * to convert to
                                     * opt_account_id_hex for use. */
  pub price: f64,
  pub amount: u32,
  // TODO , considering:
  // pub wallet_amount:u32,
  pub time: TimeStamp, //transaction_time
  pub t_type: String,  //transaction_type
  pub tag: String,
  pub manual: bool,
  pub comment: String,
}
