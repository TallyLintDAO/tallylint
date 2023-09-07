use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::env::{CanisterEnvironment, EmptyEnvironment, Environment};

use crate::common;
// pub mod user;
use crate::user::domain::UserProfile;
use crate::user::UserService;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DaoDataStorage {
    pub id: u64,
    pub users: Vec<UserProfile>,
}

impl From<DaoContext> for DaoDataStorage {
    fn from(context: DaoContext) -> Self {
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users.values().cloned());

        Self { id, users }
    }
}

pub struct DaoContext {
    pub env: Box<dyn Environment>,
    pub id: u64,
    pub user_service: UserService,
}

impl Default for DaoContext {
    fn default() -> Self {
        Self {
            env: Box::new(EmptyEnvironment {}),
            id: 10001,
            user_service: UserService::default(),
        }
    }
}

impl From<DaoDataStorage> for DaoContext {
    fn from(payload: DaoDataStorage) -> Self {
        let users: BTreeMap<Principal, UserProfile> =
            payload.users.into_iter().map(|u| (u.owner, u)).collect();

        Self {
            env: Box::new(CanisterEnvironment {}),
            id: payload.id,
            user_service: UserService { users },
        }
    }
}

#[cfg(test)]
mod tests {}
