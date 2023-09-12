use std::collections::BTreeMap;

use candid::Principal;

use super::domain::*;



/**
整个BTree功能类似于Redis的KV存储.
然后持久化整个Map实体到IC-DB里面去
*/
#[derive(Debug, Default)]
pub struct UserService {
    pub users: BTreeMap<Principal, UserProfile>,
}

impl UserService {
    pub fn insert_user(&mut self, user: UserProfile) -> Result<Principal, String> {
        let owner = user.owner;
        match self.users.get(&owner) {
            Some(_) => Err(String::from(" UserAlreadyExists")),
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

    pub fn add_wallet(&mut self, user: &Principal, info: CustomWalletInfo) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.custom_wallet_info_array.push(info);
            })
            .map(|_| true)
    }

    pub fn delete_wallet(&mut self, user: &Principal, wallet_addr: String) -> Option<bool> {
        if let Some(profile) = self.get_profile(user) {
            let custom_wallet_info_array = &profile.custom_wallet_info_array;
            for (index, custom_wallet_info) in custom_wallet_info_array.iter().enumerate() {
                if custom_wallet_info.front_end_wallet_info.addr == wallet_addr {
                    self.users
                        .get_mut(user)
                        .map(|profile| {
                            profile.custom_wallet_info_array.remove(index);
                        })
                        .map(|_| true);
                    break;
                }
            }
            return Some(true);
        } else {
            return Some(false);
        }
    }

    pub fn query_wallet_array(&mut self, user: &Principal) -> Option<Vec<CustomWalletInfo>> {
        if let Some(user_profile) = self.users.get(user) {
            return Some(user_profile.custom_wallet_info_array.clone());
        }
        return Some(Vec::new()); 
    }

    pub fn get_profile(&self, owner: &Principal) -> Option<&UserProfile> {
        self.users.get(owner)
    }
}
