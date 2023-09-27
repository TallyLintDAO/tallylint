use crate::common::env::Environment;
use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::user::domain::UserProfile;
use crate::user::service::UserService;
use crate::wallet::domain::WalletProfile;
use crate::wallet::service::WalletService;

use super::env::{CanisterEnvironment, EmptyEnvironment};

pub type WalletId = u64;

pub struct CanisterContext {
    pub env: Box<dyn Environment>,
    pub id: u64,
    pub user_service: UserService,
    pub wallet_service: WalletService,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CanisterDB {
    pub id: u64,
    pub users: Vec<UserProfile>,
    pub wallets: Vec<WalletProfile>,
}

impl Default for CanisterContext {
    fn default() -> Self {
        Self {
            env: Box::new(EmptyEnvironment {}),
            id: 10001,
            user_service: UserService::default(),
            wallet_service: WalletService::default(),
        }
    }
}

use std::num::ParseIntError;

#[allow(dead_code)]
fn integer_part(value: &str) -> Result<u64, ParseIntError> {
    let dot_pos = value.find(".").unwrap_or(value.len());
    value[..dot_pos].parse()
}

impl From<CanisterDB> for CanisterContext {
    fn from(payload: CanisterDB) -> Self {
        let users: BTreeMap<Principal, UserProfile> =
            payload.users.into_iter().map(|u| (u.owner, u)).collect();
        let wallets: BTreeMap<WalletId, WalletProfile> =
            payload.wallets.into_iter().map(|p| (p.id, p)).collect();
        Self {
            env: Box::new(CanisterEnvironment {}),
            id: payload.id,
            user_service: UserService { users },
            wallet_service: WalletService { wallets },
        }
    }
}

impl From<CanisterContext> for CanisterDB {
    fn from(context: CanisterContext) -> Self {
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        let wallets = Vec::from_iter(context.wallet_service.wallets.values().cloned());
        Self { id, users, wallets }
    }
}
