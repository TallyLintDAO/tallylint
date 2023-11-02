use std::collections::HashMap;

use ic_cdk_macros::{query, update};
use ic_stable_structures::BTreeMap;

use super::domain::*;
use super::service::WalletAddress;
use crate::common::guard::user_owner_guard;
use crate::CONTEXT;

const MAX_WALLET_NAME_LENGTH: usize = 64;

#[update(guard = "user_owner_guard")]
fn add_wallet(wallet_add_command: WalletAddCommand) -> Result<bool, String> {
    CONTEXT.with(|c| {
        if wallet_add_command.name.len() > MAX_WALLET_NAME_LENGTH {
            return Err(String::from("Wallet name exceeds maximum length 64"));
        }
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        let now = ctx.env.now();
        let id = ctx.id;
        let profile = WalletProfile {
            holder: caller,
            address: wallet_add_command.address,
            from: wallet_add_command.from,
            name: wallet_add_command.name,
            id: id,
            create_time: now,
            transactions: 0,
            last_sync_time: 0,
            last_transaction_time: 0,
        };
        match ctx.wallet_service.add_wallet(profile, caller) {
            Some(_) => {
                ctx.id += 1;
                Ok(true)
            }
            None => Err("Can not add wallet,address duplicated".to_string()),
        }
    })
}

#[update(guard = "user_owner_guard")]
fn update_wallet(wallet_update_command: WalletUpdateCommand) -> Result<bool, String> {
    CONTEXT.with(|c| {
        if wallet_update_command.name.len() > MAX_WALLET_NAME_LENGTH {
            return Err(String::from("Wallet name exceeds maximum length 64"));
        }
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        let now = ctx.env.now();
        let id: u64 = wallet_update_command.id;
        let mut profile = query_a_wallet(id).unwrap();
        // holder: caller,
        // profile.address=wallet_update_command.address;
        // profile.from=wallet_update_command.from;
        profile.name = wallet_update_command.name;
        // id: id,
        // profile.create_time=now;
        // todo
        // transactions: now,
        // last_sync_time: now,
        // last_transaction_time: now,
        match ctx.wallet_service.update_wallet(profile, caller) {
            Some(_) => Ok(true),
            None => Err("Can not update wallet".to_string()),
        }
    })
}

#[query(guard = "user_owner_guard")]
fn query_a_wallet(id: u64) -> Result<WalletProfile, String> {
    CONTEXT.with(|c| {
        let ctx = c.borrow_mut();
        let wallet = match ctx.wallet_service.query_a_wallet(id) {
            Some(wallet) => wallet,
            None => {
                return Err("wallet not exsit".to_string());
            }
        };
        return Ok(wallet);
    })
}

#[query(guard = "user_owner_guard")]
fn query_all_wallets() -> Result<Vec<WalletProfile>, Vec<WalletProfile>> {
    CONTEXT.with(|c| {
        let ctx = c.borrow_mut();
        let user = ctx.env.caller();
        let wallets = ctx.wallet_service.query_wallet_array(user);
        return Ok(wallets);
    })
}

#[update(guard = "user_owner_guard")]
fn delete_wallet(id: u64) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        ctx.wallet_service
            .delete_wallet(id)
            .ok_or(String::from("Wallet Not Found"))
    })
}

#[update(guard = "user_owner_guard")]
fn add_transaction_record(profile: RecordProfile) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        ctx.wallet_service
            .add_transaction_record(profile)
            .ok_or(String::from("Wallet Not Found"))
    })
}

// todo
#[query(guard = "user_owner_guard")]
fn wallet_hitory(
    cmd: HistoryQueryCommand,
) -> Result<HashMap<WalletAddress, Vec<RecordProfile>>, String> {
    return Err("no data".to_string());
}

// todo
// #[query(guard = "user_owner_guard")]
// fn wallet_history() {
//     // ret type: wallet_history
// }

// todo
// #[update(guard = "user_owner_guard")]
// fn sync_wallet_records() -> Result<bool, String> {
//     CONTEXT.with(|c| {

//     })
// }
