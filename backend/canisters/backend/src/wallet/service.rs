#[allow(unused_imports)]
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

use super::domain::*;

use crate::common::context::TimeStamp;

#[allow(unused_imports)]
use crate::STATE;

pub type RecordId = u64;
pub type WalletId = u64;
pub type WalletAddress = String;

/**
整个BTree功能类似于Redis的KV存储.
然后持久化整个Map实体到IC-DB里面去
*/
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct WalletService {
  pub wallets: BTreeMap<WalletId, WalletProfile>,
}
#[derive(Debug, Default)]
pub struct TransactionRecord {
  // Primary Key
  pub record_id: u64,

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

impl WalletService {
  pub fn add_wallet(
    &mut self,
    profile: WalletProfile,
    user: Principal,
  ) -> Option<String> {
    let user_wallets = self.query_wallet_array(user);
    if user_wallets
      .iter()
      .any(|wallet| wallet.address == profile.address)
    {
      return None; //add fail: wallet address already
                   // exists
    }
    let id = profile.id;
    match self.wallets.insert(id, profile) {
      Some(_) => Some("add success".to_string()), /* Wallet found and */
      // removed successfully
      None => Some("add fail".to_string()),
    }
  }

  pub fn get_addr_by_id(&mut self, id: WalletId) -> WalletAddress {
    self.wallets.get(&id).unwrap().address.clone()
  }
  //get id from wallet
  pub fn get_principal_by_id(&mut self, id: WalletId) -> Option<String> {
    self.wallets.get(&id).and_then(|wallet| wallet.principal_id.clone())
  }

  pub fn get_id_by_addr(&mut self, id: WalletAddress) -> WalletId {
    todo!()
  }
  pub fn get_all_addr_by_user(&mut self, user: Principal) -> Vec<String> {
    let addresses: Vec<String> = self
      .wallets
      .values()
      .filter(|profile| &profile.holder == &user)
      .map(|profile| profile.address.clone())
      .collect();
    addresses
  }
  pub fn get_by_addr(&mut self, addr: String) -> WalletProfile {
    self
      .wallets
      .values()
      .find(|profile| profile.address == addr)
      .expect("wallet of this addr not exsit")
      .clone()
  }

  pub fn update_wallet(
    &mut self,
    profile: WalletProfile,
    user: Principal,
  ) -> Option<String> {
    let user_wallets = self.query_wallet_array(user);

    if let Some(wallet) = user_wallets
      .iter()
      .find(|wallet| wallet.address == profile.address)
    {
      match self.wallets.insert(wallet.id, profile) {
        Some(_) => Some("update success".to_string()),
        None => None,
      }
    } else {
      return None;
    }
  }
//delete the wallet and its transactions
  pub fn delete_wallet(&mut self, id: u64) -> bool {
    match self.wallets.remove(&id) {
      Some(_) => true, /* Wallet found and removed */
      // successfully
      None => false, // Wallet not found
    }
  }

  pub fn query_a_wallet_by_id(&self, id: WalletId) -> Option<WalletProfile> {
    let wallet = self.wallets.get(&id);
    return wallet.cloned();
  }

  pub fn query_wallet_array(&self, user: Principal) -> Vec<WalletProfile> {
    let profiles: Vec<&WalletProfile> = self
      .wallets
      .values()
      .filter(|profile| &profile.holder == &user)
      .collect();
    if profiles.is_empty() {
      return Vec::new();
    } else {
      // convert Vec<&xx> to Vec<xx>
      let cloned_profiles: Vec<WalletProfile> = profiles
        .iter()
        .cloned()
        .map(|profile| (*profile).clone())
        .collect();
      return cloned_profiles;
    }
  }
  pub fn query_all_neuron(&self, user: Principal) -> Vec<WalletProfile> {
    let profiles: Vec<&WalletProfile> = self
      .wallets
      .values()
      .filter(|profile| &profile.holder == &user)
      .filter(|p| p.from == "NNS_neuron".to_string())
      .collect();
    // profiles.into_iter().filter(|p|p.from=="NNS_neuron".to_string()).
    // collect();
    if profiles.is_empty() {
      return Vec::new();
    } else {
      // convert Vec<&xx> to Vec<xx>
      let cloned_profiles: Vec<WalletProfile> = profiles
        .iter()
        .cloned()
        .map(|profile| (*profile).clone())
        .collect();
      return cloned_profiles;
    }
  }

  pub fn add_transaction_index(&self, _id: WalletId) {
    // self.wallets
  }

  pub fn new() -> Self {
    WalletService {
      wallets: BTreeMap::new(),
    }
  }
}
