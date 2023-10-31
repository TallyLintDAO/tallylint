use ic_cdk_macros::{query, update};

use super::domain::*;
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
        let id = ctx.id;
        let profile = WalletProfile {
            holder: caller,
            address: wallet_update_command.address,
            from: wallet_update_command.from,
            name: wallet_update_command.name,
            id: id,
            create_time: now,
            transactions: wallet_update_command.transactions,
            last_sync_time: wallet_update_command.last_sync_time,
            last_transaction_time: wallet_update_command.last_transaction_time,
        };
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
        let wallet = match ctx.wallet_service.query_a_wallet(id){
            Some(wallet)=>wallet,
            None=>{
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
