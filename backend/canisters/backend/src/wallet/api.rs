use std::borrow::BorrowMut;
use std::collections::HashMap;

use ic_cdk::caller;
use ic_cdk_macros::{query, update};
use ic_ledger_types::AccountIdentifier;

use super::domain::*;
use super::service::{RecordId, WalletAddress, WalletId};
use crate::common::guard::user_owner_guard;
use crate::CONTEXT;

const MAX_WALLET_NAME_LENGTH: usize = 64;
const ACCOUNT_ID_LENGTH: usize = 64;
const PRINCIPAL_ID_LENGTH: usize = 63;

#[update(guard = "user_owner_guard")]
fn add_wallet(cmd: WalletAddCommand) -> Result<bool, String> {
  CONTEXT.with(|c| {
    if cmd.name.len() > MAX_WALLET_NAME_LENGTH {
      return Err(String::from("Wallet name exceeds maximum length 64"));
    }
    if cmd.address.len() != ACCOUNT_ID_LENGTH {
      return Err(String::from("acccount_id length need to be 64"));
    }

    let if_principal: Option<String> = cmd.principal_id.clone();
    if if_principal.is_some() {
      if if_principal.clone().unwrap().len() != PRINCIPAL_ID_LENGTH {
        return Err(String::from("principal_id length need to be 63"));
      }
    }

    let mut ctx = c.borrow_mut();
    let caller = ctx.env.caller();
    let now = ctx.env.now();
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
    let caller = ctx.env.caller();
    // let now = ctx.env.now();
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
    let user = ctx.env.caller();
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

// TODO use: AddRecordCommand . front end dont need to input
// id . id gen by backend. TODO 测试 id 正常生成且不冲突
#[update(guard = "user_owner_guard")]
fn add_transaction_record(cmd: AddRecordCommand) -> Result<RecordId, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let id = ctx.id;
    let profile = TransactionB {
      id: id,

      coin_type: cmd.coin_type,
      address: cmd.address,
      price: cmd.price,
      amount: cmd.amount,
      timestamp: cmd.time,
      t_type: cmd.t_type,
      tag: cmd.tag,
      manual: cmd.manual,
      comment: cmd.comment,
      principal_id: cmd.principal_id,
      hash: cmd.hash,
      status: cmd.status,
      from: cmd.from,
      to: cmd.to,
      fee: cmd.fee,
      memo: cmd.memo,
      cost: cmd.cost,
      income: cmd.income,
      profit: cmd.profit,
    };
    let ret = ctx
      .wallet_record_service
      .add_transaction_record(profile.clone());
    match ret {
      Ok(_) => {
        ctx.id += 1;
        return Ok(profile.id);
      }
      Err(msg) => Err(msg),
    }
  })
}

#[update(guard = "user_owner_guard")]
fn delete_transaction_record(id: RecordId) -> Result<RecordId, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_record_service.delete_transaction_record(id);
    match ret {
      Ok(_) => Ok(id),
      Err(msg) => Err(msg),
    }
  })
}

#[update(guard = "user_owner_guard")]
fn edit_transaction_record(cmd: EditHistoryCommand) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let service = ctx.wallet_record_service.borrow_mut();
    let addr = service.get_addr_from_id(cmd.id);
    let ret = service.add_transaction_record(
      convert_edit_command_to_record_profile(cmd, addr),
    );
    match ret {
      Ok(_) => Ok(true),
      Err(msg) => Err(msg),
    }
  })
}

// TODO
//方法完成后，需要检查关联更新：钱包的交易记录总数，上次同步时间，
// 上次交易发生的时间 描述:户点击同步钱包按钮,调用nns或者交易所等api.
// 获得历史交易记录并存储到后端. (前端已有一部分计算代码),
// 可以选择全部搬移到后端或者前端直接把现有计算好的利润发送给后端 需要用到的api:
// nns dashboard的api可能要用到. 详见前端查询方法.
#[update(guard = "user_owner_guard")]
fn sync_transaction_record(
  data: HashMap<WalletId, Vec<TransactionF>>,
) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    for (_, record_profiles) in data {
      for record_profile in record_profiles {
        let _ = ctx
          .wallet_record_service
          .add_transaction_record(record_profile);
      }
    }
    Ok(true)
  })
}

// TODO get all wallets of records info
// 根据前端查询到的历史记录存到后端
#[query(guard = "user_owner_guard")]
fn wallet_history(
  mut cmd: HistoryQueryCommand,
) -> Result<HashMap<WalletAddress, Vec<TransactionB>>, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let mut history: HashMap<WalletAddress, Vec<TransactionB>> =
      HashMap::new();
    // query one
    if cmd.address.is_some() {
      let rec_srv = ctx.wallet_record_service.borrow_mut();
      history = rec_srv.query_one(cmd);
      if history.is_empty() {
        return Err("no records stored!".to_string());
      } else {
        return Ok(history);
      }
    }
    // query all
    // TODO . need test .
    // case1: wallet1 have addr and 3rec . w2 have 1 addr
    // and 0rec. w3 have no addr and rec. query all 3
    // wallets.
    let wal_srv = ctx.wallet_service.borrow_mut();
    let wallets = wal_srv.query_wallet_array(caller());
    let mut addrs: Vec<String> = wallets
      .iter()
      .map(|wallet| wallet.address.clone())
      .collect();
    let rec_srv = ctx.wallet_record_service.borrow_mut();
    while !addrs.is_empty() {
      let addr = addrs.pop().unwrap();
      cmd.address = Some(addr.clone());
      let rec = rec_srv.query_one(cmd.clone());
      if rec.is_empty() {
        history.insert(addr, vec![]);
        continue;
      }
      let v = rec.get(&addr).unwrap();
      history.insert(addr, v.to_vec());
    }

    if history.is_empty() {
      return Err("no records stored!".to_string());
    } else {
      return Ok(history);
    }
  })
}

fn convert_edit_command_to_record_profile(
  cmd: EditHistoryCommand,
  addr: WalletAddress,
) -> TransactionB {
  TransactionB {
    id: cmd.id,
    coin_type: cmd.coin_type,

    address: addr,
    price: cmd.price,
    amount: cmd.amount,
    timestamp: cmd.time,
    t_type: cmd.t_type,
    tag: cmd.tag,
    manual: cmd.manual,
    comment: cmd.comment,
    principal_id: None,
    hash: cmd.hash,
    status: cmd.status,
    from: cmd.from,
    to: cmd.to,
    fee: cmd.fee,
    memo: cmd.memo,
    cost: cmd.cost,
    income: cmd.income,
    profit: cmd.profit,
  }
}

#[allow(dead_code)]
fn get_account_id(hex_str: String) -> AccountIdentifier {
  let account = AccountIdentifier::from_hex(&hex_str);
  if account.is_ok() {
    return account.unwrap();
  }
  // err handle:
  let empty_account_identifier = AccountIdentifier::from_hex(
    "0000000000000000000000000000000000000000000000000000000000000000",
  )
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
      address: String::from(
        "868d0e5ed0d4a61c11c8c16e699af338058197a4e433a5b3fd582a1f31aaa5c3",
      ),
      from: String::from("Plug"),
    };
    let _cm2 = WalletAddCommand {
      principal_id: None,
      name: String::from("My Wallet"),
      address: String::from(
        "868d0e5ed0d4a61c11c8c16e699af338058197a4e433a5b3fd582a1f31aaa5c3",
      ),
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
