use ic_cdk::caller;
use ic_cdk_macros::{query, update};
use ic_ledger_types::AccountIdentifier;

use super::domain::*;

use crate::common::guard::user_owner_guard;

use crate::lifecycle::init::CONTEXT;

const MAX_WALLET_NAME_LENGTH: usize = 64;
const ACCOUNT_ID_LENGTH: usize = 64;
const PRINCIPAL_ID_LENGTH: usize = 63;
use ic_cdk::api::time;
#[update]
// #[update(guard = "user_owner_guard")]
fn add_wallet(cmd: WalletAddCommand) -> Result<bool, String> {
  CONTEXT.with(|c| {
    if cmd.name.len() > MAX_WALLET_NAME_LENGTH {
      return Err(String::from("Wallet name exceeds maximum length 64"));
    }
    if cmd.address.len() != ACCOUNT_ID_LENGTH {
      return Err(String::from("wallet_id length need to be 64"));
    }

    let if_principal: Option<String> = cmd.principal_id.clone();
    if if_principal.is_some() {
      if if_principal.clone().unwrap().len() != PRINCIPAL_ID_LENGTH {
        return Err(String::from("principal_id length need to be 63"));
      }
    }
    let now = time();
    let mut ctx = c.borrow_mut();
    let caller = caller();
    let id = ctx.id;
    let mut profile = WalletProfile {
      holder: caller,
      address: cmd.address,
      from: cmd.from,
      name: cmd.name,
      id: id,
      create_time: now,
      transactions: 0,
      last_sync_time: 0,
      last_transaction_time: 0,
      principal_id: None,
    };
    if if_principal.is_some() {
      profile.principal_id = if_principal.clone();
    }
    match ctx.wallet_service.add_wallet(profile, caller) {
      Some(_) => {
        ctx.id += 1;
        Ok(true)
      }
      None => Err("Can not add wallet,address duplicated".to_string()),
    }
  })
}

// TODO records things TODO .
// TODO  字段还未全部实现方法.只有定义.
#[update(guard = "user_owner_guard")]
fn update_wallet(cmd: WalletUpdateCommand) -> Result<bool, String> {
  CONTEXT.with(|c| {
    if cmd.name.len() > MAX_WALLET_NAME_LENGTH {
      return Err(String::from("Wallet name exceeds maximum length 64"));
    }
    let mut ctx = c.borrow_mut();
    let caller = caller();
    // let now = time();
    let id: u64 = cmd.id;
    let ret = ctx.wallet_service.query_a_wallet(id);
    if ret.is_none() {
      return Err("wallet not exist".to_string());
    }
    let mut profile = ret.unwrap();
    // holder: caller,
    // profile.address=wallet_update_command.address;
    // profile.from=wallet_update_command.from;
    profile.name = cmd.name;
    profile.from = cmd.from;
    // id: id,
    // profile.create_time=now;
    // TODO
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
// #[query]
fn query_a_wallet(id: u64) -> Result<WalletProfile, String> {
  CONTEXT.with(|c| {
    let ctx = c.borrow_mut();
    let wallet = match ctx.wallet_service.query_a_wallet(id) {
      Some(wallet) => wallet.clone(),
      None => {
        return Err("wallet not exsit".to_string());
      }
    };
    return Ok(wallet.clone());
  })
}

#[query(guard = "user_owner_guard")]
fn query_all_wallets() -> Result<Vec<WalletProfile>, Vec<WalletProfile>> {
  CONTEXT.with(|c| {
    let ctx = c.borrow_mut();
    let user = caller();
    let wallets = ctx.wallet_service.query_wallet_array(user);
    return Ok(wallets);
  })
}

#[update(guard = "user_owner_guard")]
fn delete_wallet(id: u64) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    ctx
      .wallet_service
      .delete_wallet(id)
      .ok_or(String::from("Wallet Not Found"))
  })
}

#[allow(dead_code)]
fn get_account_id(hex_str: String) -> AccountIdentifier {
  let account = AccountIdentifier::from_hex(&hex_str);
  if account.is_ok() {
    return account.unwrap();
  }
  // err handle:
  let empty_account_identifier =
    AccountIdentifier::from_hex("0000000000000000000000000000000000000000000000000000000000000000")
      .unwrap();
  return empty_account_identifier;
}

#[cfg(test)]
// #[cfg(target_arch = "wasm32")]
mod tests {
  use super::*;

  #[test]
  fn test_add_wallet() {
    let should_ok_cmd = WalletAddCommand {
      principal_id: Some(String::from(
        "b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae",
      )),
      name: String::from("My Wallet"),
      address: String::from("868d0e5ed0d4a61c11c8c16e699af338058197a4e433a5b3fd582a1f31aaa5c3"),
      from: String::from("Plug"),
    };
    let _cm2 = WalletAddCommand {
      principal_id: None,
      name: String::from("My Wallet"),
      address: String::from("868d0e5ed0d4a61c11c8c16e699af338058197a4e433a5b3fd582a1f31aaa5c3"),
      from: String::from("NNS"),
    };
    // not work locally : https://forum.dfinity.org/t/guys-how-do-you-debug-your-rust-backend-canister/22965
    // TODO :maybe spilit rust logic and ic-logic
    // ic-chain local replica(a rust binnary running
    // distributed system ) supply a runtime for
    // canister(wasm code). TODO 如果可以debug ic-replica.
    // 那么有可能可以联合 rust-logic 和ic-logic TODO
    // 如果都不行.
    // 可以试试采用logging系统来记录程序运行.
    let _t = ic_cdk::api::time();
    let _c = ic_cdk::caller();
    let _can_id = ic_cdk::id();

    let result = add_wallet(should_ok_cmd.clone());
    // Check the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
    // Create a sample WalletAddCommand

    // Additional assertions to verify the changes in the
    // context or wallet_service if applicable
  }
}
