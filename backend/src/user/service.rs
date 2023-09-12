use std::collections::BTreeMap;

use candid::Principal;

use super::{
    domain::{UserEditCommand, UserProfile},
};

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

    pub fn edit_user(
        &mut self,
        cmd: UserEditCommand,
        principal: &Principal,
    ) -> Result<bool, String> {
        match self.users.get_mut(principal) {
            None => Err(String::from("user not found")),
            Some(user) => cmd.build_profile(user),
        }
    }

    pub fn update_wallet(&mut self, user: &Principal, wallet: Principal) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.wallet_principal = Some(wallet);
            })
            .map(|_| true)
    }

    pub fn delete_wallet(&mut self, user: &Principal) -> Option<bool> {
        self.users
            .get_mut(user)
            .map(|profile| {
                profile.wallet_principal = None;
            })
            .map(|_| true)
    }
}
