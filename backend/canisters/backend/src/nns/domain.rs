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

#[derive(Debug, Clone, CandidType, Serialize, Deserialize, Default)]
pub struct NeuronAddCommand {
  pub address: String,
  pub from: String,
  pub name: String,
  // pub test1: String,
}
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]  this is strict deserial mode . not need.
pub struct NeuronProfile {
  pub owner: Principal,
  pub name: String,
  pub id: u64,
  pub address: String,
  pub create_time: u64,
  // newlly add . update with DB deserilize err. the odd bin DB file fail to
  // find this field . shoud ignore it .
  // also : #[serde(default="a_function")] to give it a custom val.

  // This attr will use default value(maybe 0) if missing during
  // deserialization
  // #[serde(default = "default_update_time")]
  // pub update_time: u64,
  /* TODO need let dfx deploy backend print deserialize err info ! in order
   * to */
}

fn default_update_time() -> u64 {
  0
}
