use std::collections::{BTreeMap, HashMap};

use candid::{CandidType, Principal};
use ic_cdk::caller;

use super::domain::*;

use crate::common::context::TimeStamp;

#[allow(unused_imports)]
use crate::CONTEXT;

pub type WalletId = u64;
pub type RecordId = u64;
pub type WalletAddress = String;

/**
整个BTree功能类似于Redis的KV存储.
然后持久化整个Map实体到IC-DB里面去
*/
#[derive(Debug, Default)]
pub struct WalletService {
    pub wallets: BTreeMap<WalletId, WalletProfile>,
}

#[derive(Debug, Default)]
pub struct WalletRecordService {
    pub records: BTreeMap<RecordId, RecordProfile>,
}

#[derive(Debug, Default)]
pub struct TransactionRecord {
    // Primary Key
    pub record_id: u64,

    pub price: f64,
    pub amount: u32,
    // todo , considering:
    // pub wallet_amount:u32,
    pub time: TimeStamp, //transaction_time
    pub t_type: String,  //transaction_type
    pub tag: String,
    pub manual: bool,
    pub comment: String,
}

impl WalletService {
    pub fn add_wallet(&mut self, profile: WalletProfile, user: Principal) -> Option<String> {
        let user_wallets = self.query_wallet_array(user);
        if user_wallets
            .iter()
            .any(|wallet| wallet.address == profile.address)
        {
            return None; //add fail: wallet address already exists
        }
        let id = profile.id;
        match self.wallets.insert(id, profile) {
            Some(_) => Some("add success".to_string()), // Wallet found and removed successfully
            None => Some("add fail".to_string()),
        }
    }

    pub fn update_wallet(&mut self, profile: WalletProfile, user: Principal) -> Option<String> {
        let user_wallets = self.query_wallet_array(user);
        let mut id = 0;
        if let Some(wallet) = user_wallets
            .iter()
            .find(|wallet| wallet.address == profile.address)
        {
            id = wallet.id;
            match self.wallets.insert(id, profile) {
                Some(_) => Some("update success".to_string()),
                None => None,
            }
        } else {
            return None;
        }
    }

    pub fn delete_wallet(&mut self, id: u64) -> Option<bool> {
        match self.wallets.remove(&id) {
            Some(_) => Some(true), // Wallet found and removed successfully
            None => Some(false),   // Wallet not found
        }
    }

    pub fn query_a_wallet(&self, id: WalletId) -> Option<WalletProfile> {
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

    #[allow(dead_code)]
    pub fn new() -> Self {
        WalletService {
            wallets: BTreeMap::new(),
        }
    }
}

impl WalletRecordService {
    // todo
    pub fn add_transaction_record(&mut self, profile: RecordProfile) -> Result<bool, String> {
        let id = profile.id;
        if self.records.contains_key(&id) {
            return Err("transaction record already exsit".to_string());
        }

        self.records.insert(profile.id, profile);

        if self.records.contains_key(&id) {
            return Ok(true);
        } else {
            return Err("Insert fail. may heap overflow".to_string());
        }
    }
    pub fn delete_transaction_record(&mut self, id: RecordId) -> Result<bool, String> {
        if !self.records.contains_key(&id) {
            return Err("transaction record not exsit".to_string());
        }

        self.records.remove(&id);

        if !self.records.contains_key(&id) {
            return Ok(true);
        } else {
            return Err("remove fail. still exsit".to_string());
        }
    }
    pub fn get_addr_from_id(&self, id: RecordId) -> WalletAddress {
        self.records.get(&id).unwrap().address.clone()
    }
    pub fn wallet_history(
        &self,
        cmd: HistoryQueryCommand,
    ) -> Result<HashMap<WalletAddress, Vec<RecordProfile>>, String> {
        if cmd.address.is_some() { // query one 
            let addr=cmd.address.unwrap().clone();
            let records=self.query_a_wallet_his(addr.clone());
            let mut res=HashMap::new();
            res.insert(addr.clone(), records);
            return Ok(res);
        }else{ //query all 
            // let wallets=WalletService::query_wallet_array(self,caller());
            // from ctx or ? 
            
        }
        return Err("nothing".to_string());
    }

    pub fn query_a_wallet_his(&self,addr:WalletAddress)->Vec<RecordProfile>{
        let records=self.records
        .values()
        .filter(|record| record.address == addr)
        .cloned()
        .collect();
        return records;
    }
    

}
