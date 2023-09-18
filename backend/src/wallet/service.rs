use std::collections::BTreeMap;

use candid::Principal;

use super::domain::*;

/**
整个BTree功能类似于Redis的KV存储.
然后持久化整个Map实体到IC-DB里面去
*/
use crate::CONTEXT;

type WalletId = u64;
#[derive(Debug, Default)]
pub struct WalletService {
    pub wallets: BTreeMap<WalletId, WalletProfile>,
}

impl WalletService {
    pub fn add_wallet(&mut self,  profile: WalletProfile) -> Option<String> {
            let id = profile.id;  
            match self.wallets.insert(id, profile){
                Some(_) => Some("add success".to_string()),  // Wallet found and removed successfully
                None => Some("add fail".to_string()),  
            }
    }

    pub fn delete_wallet(&mut self, id: u64) -> Option<bool> {
        match self.wallets.remove(&id) {
            Some(_) => Some(true),  // Wallet found and removed successfully
            None => Some(false),    // Wallet not found
        }
    }

    pub fn query_wallet_array(&mut self, user: Principal) -> Option<Vec<WalletProfile>> {
        let profiles: Vec<&WalletProfile> = self
            .wallets
            .values()
            .filter(|profile| &profile.holder == &user)
            .collect();
        if profiles.is_empty() {
            None
        } else {
            let cloned_profiles: Vec<WalletProfile> = profiles
                .iter()
                .cloned()
                .map(|profile| (*profile).clone())
                .collect();
            Some(cloned_profiles)
        }
    }


    pub fn new() -> Self {
        WalletService {
            wallets: BTreeMap::new(),
        }
    }
}
