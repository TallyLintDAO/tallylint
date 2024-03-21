pub(crate) use candid::CandidType;

use super::service::{TransactionId, WalletAddress, WalletId};
use serde::{Deserialize, Serialize};

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
// TODO multi profit type
#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct DetailsB {
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
  // pub profit_method: String,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Currency {
  pub decimals: u8,  //代币精度,多少个0
  pub symbol: String, //代币符号，例如'ICP'，'CHAT'
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionB {
  //
  // backend autogen:
  pub id: TransactionId,
  //
  pub hash: String,
  pub timestamp: u64, //this is ns format usigned 64bit
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub details: Details,
  pub memo: String,
  pub address: WalletAddress,

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

impl TransactionB {
  pub fn trim(self) -> SimpleTransaction {
    SimpleTransaction {
      id: self.id,
      hash: self.hash,
      timestamp: self.timestamp,
      t_type: self.t_type,
      details: self.details,
      tag: self.tag,
      manual: self.manual,
      comment: self.comment,
    }
  }
}
