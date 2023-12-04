use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::wallet::service::WalletId;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct NeuronUpdateCommand {
  // muttable
  pub name: String,

  // immutable . for locate the wallet
  pub id: WalletId, /*when update , specify id .
                     * pub transactions:u64,
                     * pub last_sync_time:u64,
                     * pub last_transaction_time:u64, */
}

#[derive(Debug, Clone, CandidType,Serialize,  Deserialize, Default)]
pub struct NeuronAddCommand {
  pub address: String,
  pub from: String,
  pub name: String,
}
#[derive(Debug, Clone, CandidType,Serialize,  Deserialize)]
pub struct NeuronProfile {
  pub owner: Principal, // 用户 Principal
  pub name: String,
  pub id: u64,
  pub address: String,
  pub create_time: u64,
}
