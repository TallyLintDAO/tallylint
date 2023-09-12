use std::collections::BTreeMap;

use candid::Principal;

use super::{
    domain::{ UserProfile, CustomWalletInfo},
};



/**
 整个BTree功能类似于Redis的KV存储.
 然后持久化到IC-DB里面去
 */
#[derive(Debug, Default)]
pub struct UserService {
    pub users: BTreeMap<Principal, UserProfile>,
}

impl UserService {
    pub fn insert_user(&mut self, user: UserProfile) -> Result<Principal, String> {
        let owner = user.owner;
        match self.users.get(&owner) {
            Some(_) => Err(String::from(" UserAlreadyExists") ),
            None => {
                self.users.insert(owner, user);
                Ok(owner)
            }
        }
    }

    pub fn is_owner(&self, caller: &Principal) -> bool {
        matches!(self.users.get(caller), Some(u) if u.owner == *caller)
    }

    pub fn get_user(&self, principal: &Principal) -> Option<UserProfile> {
        self.users.get(principal).cloned()
    }

    // pub fn update_wallet(&mut self, user: &Principal, info:Vec<CustomWalletInfo>) -> Option<bool> {
    pub fn update_wallet(&mut self, user: &Principal, info:CustomWalletInfo) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.custom_wallet_info = Some(info);
            })
            .map(|_| true)
    }

    pub fn delete_wallet(&mut self, user: &Principal) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.custom_wallet_info =  None;
            })
            .map(|_| true)
    }
}
