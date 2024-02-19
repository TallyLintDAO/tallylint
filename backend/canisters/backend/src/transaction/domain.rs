pub(crate) use candid::{CandidType, Principal};

use crate::common::context::TimeStamp;

use super::service::{RecordId, WalletAddress, WalletId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Wallet {
  walletid: u64,
  wallet_history: Vec<TransactionF>,
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
