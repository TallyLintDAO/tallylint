#[allow(unused_imports)]
use candid::{CandidType, Principal};
use serde::Serialize;
use std::collections::BTreeMap;

#[allow(unused_imports)]
use serde::Deserialize;

use super::domain::*;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct UserService {
  pub users: BTreeMap<Principal, UserProfile>,
  pub configs: BTreeMap<Principal, UserConfig>,
}

impl UserService {
  pub fn insert_user(
    &mut self,
    user: UserProfile,
  ) -> Result<UserProfile, String> {
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

  #[allow(dead_code)]
  pub fn get_profile(&self, owner: &Principal) -> Option<&UserProfile> {
    self.users.get(owner)
  }

  pub fn user_quantity(&self) -> u32 {
    return self.users.len().try_into().unwrap_or_default();
  }
  #[allow(dead_code)]
  pub fn new() -> Self {
    UserService {
      users: BTreeMap::new(),
      configs: BTreeMap::new(),
    }
  }
}
