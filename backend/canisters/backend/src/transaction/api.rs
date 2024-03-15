use std::borrow::BorrowMut;
use std::collections::HashMap;

use ic_cdk::caller;
use ic_cdk_macros::{query, update};

use super::domain::*;
use super::service::{HistoryQueryCommand, TransactionId, WalletAddress};
use crate::common::context::{get_caller, now};
use crate::common::guard::user_owner_guard;
use crate::common::times::ms_float_to_ns;
use crate::{TransactionB, CONTEXT};

// TODO use: AddRecordCommand . front end dont need to input
// id . id gen by backend. TODO 测试 id 正常生成且不冲突
#[update(guard = "user_owner_guard")]
fn add_transaction(mut data: TransactionB) -> Result<TransactionId, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    ctx.id = ctx.id + 1;
    let id = ctx.id;
    data.id = id;
    let id = data.id;
    let ret = ctx.wallet_record_service.add_transaction_impl(data.clone());
    match ret {
      Ok(_) => {
        // TODO save the id as fast bmap index into the wallet struct.
        // ctx.index_service.add_transaction_index(id);
        return Ok(id);
      }
      Err(msg) => Err(msg),
    }
  })
}

#[update(guard = "user_owner_guard")]
fn delete_transaction(id: TransactionId) -> Result<TransactionId, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_record_service.delete_transaction_impl(id);
    match ret {
      Ok(_) => {
        // TODO delete the id as fast bmap index into the wallet struct.

        Ok(id)
      }
      Err(msg) => Err(msg),
    }
  })
}

// TODO get all wallets of records info
// many work todo to different query
#[query(guard = "user_owner_guard")]
fn query_wallet_transactions(
  cmd: HistoryQueryCommand,
) -> Result<HashMap<WalletAddress, Vec<TransactionB>>, String> {
  CONTEXT.with(|c| {
    let ctx = c.borrow_mut();
    let mut history: HashMap<WalletAddress, Vec<TransactionB>> = HashMap::new();

    for addr in cmd.address {
      let rec = ctx.wallet_record_service.query_one_wallet(addr);
      for (k, v) in rec {
        history.insert(k, v);
      }
    }
    return Ok(history);
  })
}

#[update(guard = "user_owner_guard")]
fn update_transaction(mut data: TransactionB) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    ctx.id = ctx.id + 1;
    let id = ctx.id;
    data.id = id;
    let ret = ctx.wallet_record_service.add_transaction_impl(data);
    match ret {
      Ok(_) => {
        // TODO update the id as fast bmap index into the wallet struct.
        Ok(true)
      }
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
  cmd: Vec<SyncTransactionCommand>,
) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    for one_wallet in cmd {
      for one_rec in one_wallet.history.clone() {
        ctx.id = ctx.id + 1;
        let id = ctx.id;
        let _ret = ctx.transaction_service.add_transaction_record(id, one_rec);
      }
      let mut wallet_profile = ctx
        .wallet_service
        .query_a_wallet(one_wallet.walletId)
        .expect("no such wallet");
      wallet_profile.last_sync_time = now();
      wallet_profile.transactions = one_wallet.history.len() as u64;
      wallet_profile.last_transaction_time =
        ms_float_to_ns(one_wallet.history.get(0).unwrap().clone().timestamp);
      ctx
        .wallet_service
        .update_wallet(wallet_profile, get_caller());
    }
    Ok(true)
  })
}
// TODO:
// fn filt_by_tag(
