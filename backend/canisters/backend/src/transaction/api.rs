use std::collections::HashMap;

use ic_cdk_macros::{query, update};

use super::domain::*;
use super::service::{TransactionId, WalletAddress};
use crate::common::context::{get_caller, now};
use crate::common::guard::admin_guard;
use crate::common::guard::user_owner_guard;
use crate::common::times::timestamp_ms_float_to_ns;
use crate::wallet::domain::HistoryQueryCommand;
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
    let ret = ctx.wallet_transc_srv.add_transaction_impl(data.clone());
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
    let ret = ctx.wallet_transc_srv.delete_transaction_impl(id);
    match ret {
      Ok(_) => {
        // TODO delete the id as fast bmap index into the wallet struct.

        Ok(id)
      }
      Err(msg) => Err(msg),
    }
  })
}

#[query(guard = "user_owner_guard")]
fn query_wallet_transactions(
  cmd: HistoryQueryCommand,
) -> Vec<SimpleTransaction> {
  CONTEXT.with(|c| {
    let ctx = c.borrow();
    let mut all_transactions = Vec::new();

    // !get all recs
    for addr in cmd.address {
      let rec = ctx
        .wallet_transc_srv
        .query_one_wallet(addr.clone())
        .get(&addr)
        .cloned()
        .unwrap_or(Vec::new());
      all_transactions.extend(rec);
    }

    // !delete unwant field
    let mut simple_trans: Vec<SimpleTransaction> = all_transactions
      .into_iter()
      .map(TransactionB::trim)
      .collect();

    // !filter if time range
    simple_trans.retain(|transaction| {
      transaction.timestamp >= cmd.from_time
        && transaction.timestamp <= cmd.to_time
    });

    // ! sort if need
    if cmd.sort_method.is_none() {
      return simple_trans;
    };
    let method = cmd.sort_method.unwrap();
    if method == String::from("date-asc") {
      simple_trans.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
    if method == String::from("date--desc") {
      simple_trans.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }
    if method == String::from("profit-asc") {
      simple_trans.sort_by(|a, b| {
        a.details
          .profit
          .partial_cmp(&b.details.profit)
          .unwrap_or(std::cmp::Ordering::Equal)
      });
    }
    if method == String::from("profit-desc") {
      simple_trans.sort_by(|a, b| {
        b.details
          .profit
          .partial_cmp(&a.details.profit)
          .unwrap_or(std::cmp::Ordering::Equal)
      });
    }

    return simple_trans;
  })
}


#[query(guard = "admin_guard")]
fn query_all_transactions(
) -> Result<HashMap<TransactionId, TransactionB>, String> {
  CONTEXT.with(|c| {
    let ctx = c.borrow_mut();
    let rec = ctx.wallet_transc_srv.query_all_transactions();
    return Ok(rec);
  })
}

#[update(guard = "user_owner_guard")]
fn update_transaction(data: TransactionB) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_transc_srv.update_transaction_impl(data);
    match ret {
      Ok(_) => Ok(true),
      Err(msg) => Err(msg),
    }
  })
}

#[query(guard = "user_owner_guard")]
fn query_one_transaction(id: TransactionId) -> Result<TransactionB, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_transc_srv.query_one(id);
    match ret {
      Ok(t) => Ok(t),
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
// #[update(guard = "user_owner_guard")]
#[update]
fn sync_transaction_record(
  cmd: Vec<SyncTransactionCommand>,
) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    for one_wallet in cmd {
      // insert records
      for one_rec in one_wallet.history.clone() {
        ctx.id = ctx.id + 1;
        let id = ctx.id;
        let _ret = ctx.trans_f_srv.add_transaction_record(id, one_rec.clone());

        // copy transF to transB
        ctx.id = ctx.id + 1;
        let id2 = ctx.id;
        let trans_b = convert_trans_f_to_trans_b(one_rec, id2);
        let _ = ctx.wallet_transc_srv.add_transaction_impl(trans_b);
      }

      // update wallet info
      let mut wallet_profile = ctx
        .wallet_service
        .query_a_wallet(one_wallet.walletId)
        .expect("no such wallet");
      wallet_profile.last_sync_time = now();
      wallet_profile.transactions = one_wallet.history.len() as u64;
      wallet_profile.last_transaction_time = timestamp_ms_float_to_ns(
        one_wallet.history.get(0).unwrap().clone().timestamp,
      );
      ctx
        .wallet_service
        .update_wallet(wallet_profile, get_caller());
    }

    Ok(true)
  })
}

fn convert_trans_f_to_trans_b(
  trans_f: TransactionF,
  id: TransactionId,
) -> TransactionB {
  let address = match trans_f.t_type.as_str() {
    "SEND" => trans_f.details.from.clone(),
    "RECEIVE" => trans_f.details.to.clone(),
    _ => WalletAddress::default(), // You can handle other cases here
  };

  TransactionB {
    id,
    hash: trans_f.hash,
    timestamp: timestamp_ms_float_to_ns(trans_f.timestamp),
    t_type: trans_f.t_type,
    walletName: trans_f.walletName,
    details: trans_f.details.clone(),
    principal_id: None,
    memo: String::new(),
    address,
    tag: Vec::new(),
    manual: false,
    comment: String::new(),
  }
}

