use std::borrow::BorrowMut;
use std::collections::HashMap;

use ic_cdk::caller;
use ic_cdk_macros::{query, update};

use super::domain::*;
use super::service::{
  AddRecordCommand, EditHistoryCommand, HistoryQueryCommand, RecordId,
  WalletAddress, WalletId,
};
use crate::common::guard::user_owner_guard;
use crate::{TransactionB, CONTEXT};

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
        let id = ctx.id;
        let ret = ctx
          .transaction_service
          .add_transaction_record(id, record_profile);
        if ret.is_ok() {
          ctx.id += 1;
        }
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
    let mut history: HashMap<WalletAddress, Vec<TransactionB>> = HashMap::new();
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
