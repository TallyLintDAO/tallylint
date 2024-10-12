pub(crate) use candid::CandidType;

use crate::wallet::service::WalletId;

use super::service::WalletAddress;
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
  #[serde(default)]
  pub wid: WalletId,
  pub hash: String,
  pub timestamp: f64, // TODO check ns or ms as unit
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  // pub walletName: String,
  pub details: Details,
}

pub type TransactionId = u64;

// TODO multi profit type
//FIXME无用结构体，需要删除
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

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionB {
  //
  // backend autogen:
  pub id: u64,
  #[serde(default)]
  pub wid: WalletId,
  //
  pub hash: String,
  pub timestamp: u64, /* !front end pass in its f64 ms . store in rust us
                       * u64 ms. trunc after decial digits */
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub details: Details,
  pub memo: String,
  pub address: WalletAddress,
  pub tag: Option<String>,
  // donation loan_fee margin_fee tax loan_payment
  // margin_payment realted_PL gift lost
  pub manual: bool, // if this trasac is manual import
  pub comment: String,
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
  pub decimals: u8,   //代币精度,多少个0
  pub symbol: String, //代币符号，例如'ICP'，'CHAT'
}
//FIXME 该数据结构完全无用，需要删除
#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct SimpleTransaction {
  //
  // backend autogen:
  pub id: WalletId,
  //
  pub hash: String,
  pub timestamp: u64, //this is ms format usigned 64bit
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  pub details: Details,
  pub tag: Option<String>,
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
//FIXME 该数据结构完全无用，需要删除
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
//FIXME 该数据结构完全无用，需要删除
pub struct TransactionForTax {
  pub action: String,
  pub quantity: f64,
  pub price: f64,
  pub profit: Option<f64>,
}
impl From<TransactionB> for TransactionForTax {
  fn from(transaction_b: TransactionB) -> Self {
    let action = match transaction_b.t_type.as_str() {
      "SEND" => "sell".to_string(),
      "RECEIVE" => "buy".to_string(),
      _ => "ERROR!".to_string(),
    };

    TransactionForTax {
      action,
      quantity: transaction_b.details.amount,
      price: transaction_b.details.price,
      profit: Some(transaction_b.details.profit),
    }
  }
}



#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct MySummary {
  pub capital_gain_or_loss: f64,
  pub other_gain: f64,
  pub income: f64,
  pub costs_expenses: f64,
  pub gifts_dotations_lost_coins: f64,
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
  pub struct Test1 {
    pub id: u64,
    pub tag: Option<String>,
  }

  #[test]
  fn test_transaction_b_serialization() {
    let transaction_b = Test1 {
      id: 1,

      tag: Some("donation".to_string()),
    };

    let data = serde_json::to_string(&transaction_b).unwrap();
    eprint!("{}", data);
    assert_eq!(1, 0);
    // {"id":1,"tag":"donation"}
  }
}
