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
    pub fn insert_user(&mut self, user: UserProfile) -> Result<UserProfile, String> {
        let owner = user.owner;
        match self.users.get(&owner) {
            Some(_) => Err(String::from(" UserAlreadyExists")),
            None => {
                self.users.insert(owner, user.clone());
                Ok(user)
            }
        }
    }

    pub fn is_owner(&self, caller: &Principal) -> bool {
        matches!(self.users.get(caller), Some(u) if u.owner == *caller)
    }

    pub fn get_user(&self, principal: &Principal) -> Option<UserProfile> {
        self.users.get(principal).cloned()
    }

    pub fn add_wallet(&mut self, u_principal: &Principal, info: FullWalletInfo) -> Option<String> {
        //check map.get(K) null pointer:
        // let user = match self.users.get_mut(u_principal) {
        //     Some(user) => user,         // u is a mutable reference to the user
        //     None => return Some("user not exist".to_string()),
        // };
        // let wallet_addr=info.front_end_wallet_info.addr;
        // if user.custom_wallet_info_array.contains(&info){
        //     return Some("wallet dupicated".to_string()); // Duplicate wallet address found, return false
        // }
        self.users.get_mut(u_principal).map(|profile| {
            profile.full_wallet_info_array.push(info);
            return "success";
        });
        return Some("add fail of map push".to_string());
    }

    pub fn delete_wallet(&mut self, user: &Principal, wallet_addr: String) -> Option<bool> {
        if let Some(profile) = self.get_profile(user) {
            let custom_wallet_info_array = &profile.full_wallet_info_array;
            for (index, custom_wallet_info) in custom_wallet_info_array.iter().enumerate() {
                if custom_wallet_info.wallet_info.address == wallet_addr {
                    self.users
                        .get_mut(user)
                        .map(|profile| {
                            profile.full_wallet_info_array.remove(index);
                        })
                        .map(|_| true);
                    return Some(true);
                }
            }
            return Some(false);
        } else {
            return Some(false);
        }
    }

    pub fn query_wallet_array(&mut self, user: &Principal) -> Option<Vec<FullWalletInfo>> {
        if let Some(user_profile) = self.users.get(user) {
            return Some(user_profile.full_wallet_info_array.clone());
        }
        return Some(Vec::new());
    }

    pub fn get_profile(&self, owner: &Principal) -> Option<&UserProfile> {
        self.users.get(owner)
    }

    pub fn user_quantity(&self) -> u32 {
        return self.users.len().try_into().unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_owner() {
        // Create a sample user with the caller as the owner
        let caller =
            Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae")
                .unwrap();
        let owner =
            Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae")
                .unwrap();
        let users = BTreeMap::new();
        let user_service = UserService { users };

        assert!(user_service.is_owner(&caller));
        // Test when the caller is the owner
        assert_eq!(caller, owner);
    }
}
