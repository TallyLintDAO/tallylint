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
  // ! important annotation for deserialize. if json not contains this, will
  // fill this with a empty one in rust code
  #[serde(default = "BTreeMap::new")]
  pub configs: BTreeMap<String, UserConfig>,
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

  pub fn add_config(&mut self, owner: &Principal, data: UserConfig) -> String {
    self.configs.insert(owner.to_string(), data);
    return "add ok".to_string();
  }

  pub fn get_config(&mut self, owner: &Principal) -> UserConfig {
    // Try to get the config
    match self.configs.get(&owner.to_string()) {
      // If the config exists, return it
      Some(config) => config.clone(),
      // If the config does not exist, set a default config and return it
      None => {
        let default_config = UserConfig::new("fifo".to_string(), Vec::new());
        self.update_config(&owner, default_config.clone());
        default_config
      }
    }
  }

  pub fn update_config(
    &mut self,
    owner: &Principal,
    data: UserConfig,
  ) -> String {
    self.configs.insert(owner.to_string(), data);
    return "update_config ok".to_string();
  }

  pub fn delete_config(&mut self, owner: &Principal) -> String {
    self.configs.remove(&owner.to_string());
    return "delete_config ok".to_string();
  }

  #[allow(dead_code)]
  pub fn new() -> Self {
    UserService {
      users: BTreeMap::new(),
      configs: BTreeMap::new(),
    }
  }
}
