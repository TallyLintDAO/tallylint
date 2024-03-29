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
  pub hash: String,
  pub timestamp: f64, // TODO check ns or ms as unit
  pub t_type: String, //  transaction type : "SEND", "RECEIVE"
  // pub walletName: String,
  pub details: Details,
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

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionB {
  //
  // backend autogen:
  pub id: WalletId,
  //
  pub hash: String,
  pub timestamp: u64, //front end pass in its f64 ms . store in rust us u64 ms
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

#[allow(non_snake_case)]
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct SimpleTransaction {
  //
  // backend autogen:
  pub id: WalletId,
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

pub struct WalletForTax {
  transactions: Vec<TransactionForTax>,
}
impl WalletForTax {
  fn new() -> Self {
    WalletForTax {
      transactions: Vec::new(),
    }
  }

  fn buy(&mut self, quantity: f64, price: f64) {
    self.transactions.push(TransactionForTax {
      action: "sell".to_string(),
      quantity,
      price,
      profit: None,
    });
  }

  fn sell_lifo(&mut self, quantity: f64, price: f64) -> f64 {
    let mut remaining = quantity;
    let mut gain_or_loss = 0.0;

    while remaining > 0.0 && !self.transactions.is_empty() {
      let transaction = &mut self.transactions.last_mut().unwrap();
      if transaction.quantity <= remaining {
        gain_or_loss += (price - transaction.price) * transaction.quantity;
        remaining -= transaction.quantity;
        self.transactions.pop();
      } else {
        gain_or_loss += (price - transaction.price) * remaining;
        transaction.quantity -= remaining;
        remaining = 0.0;
      }
    }

    gain_or_loss
  }
  fn sell_fifo(&mut self, quantity: f64, price: f64) -> f64 {
    let mut remaining = quantity;
    let mut gain_or_loss = 0.0;

    while remaining > 0.0 && !self.transactions.is_empty() {
      let transaction = &mut self.transactions[0];
      if transaction.quantity <= remaining {
        gain_or_loss += (price - transaction.price) * transaction.quantity;
        remaining -= transaction.quantity;
        self.transactions.remove(0);
      } else {
        gain_or_loss += (price - transaction.price) * remaining;
        transaction.quantity -= remaining;
        remaining = 0.0;
      }
    }

    gain_or_loss
  }
}

pub fn calculate_gain_or_loss(
  transactions: Vec<TransactionForTax>,
  method: String,
) -> Vec<TransactionForTax> {
  let mut wft = WalletForTax::new();
  let mut processed_transactions = Vec::new();

  for mut transaction in transactions {
    if transaction.action == "buy" {
      wft.buy(transaction.quantity, transaction.price);
      processed_transactions.push(transaction);
    } else if transaction.action == "sell" {
      match method.as_str() {
        "fifo" => {
          transaction.profit =
            Some(wft.sell_fifo(transaction.quantity, transaction.price));
          processed_transactions.push(transaction);
        }
        "lifo" => {
          transaction.profit =
            Some(wft.sell_lifo(transaction.quantity, transaction.price));
          processed_transactions.push(transaction);
        }
        _ => {
          // TODO should throw err here !
          return Vec::new();
        }
      }
    }
  }

  processed_transactions
}

#[allow(non_snake_case)]
pub fn map_taxTrans_to_transB(
  trans_tax: Vec<TransactionForTax>,
  mut trans_b: Vec<TransactionB>,
) -> Vec<TransactionB> {
  for (trans_tax, trans_b) in trans_tax.iter().zip(trans_b.iter_mut()) {
    trans_b.t_type = trans_tax.action.clone();
    trans_b.details.amount = trans_tax.quantity;
    trans_b.details.price = trans_tax.price;
    if let Some(profit) = trans_tax.profit {
      trans_b.details.profit = profit;
    }
  }
  trans_b
}
