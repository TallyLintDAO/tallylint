use std::collections::HashMap;
use std::collections::VecDeque;

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
    let ret = ctx.wallet_transc_srv.delete_transaction_by_id_impl(id);
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
fn query_all_wallet_transactions(
  cmd: HistoryQueryCommand,
) -> Vec<SimpleTransaction> {
  CONTEXT.with(|c| {
    let ctx = c.borrow();
    let mut all_transactions = Vec::new();
    
    // !get all recs
    for addr in cmd.address {
      let rec = ctx
        .wallet_transc_srv
        .query_one_wallet_trans(addr.clone())
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
    if cmd.from_time != 0 && cmd.to_time != 0 {
      simple_trans.retain(|transaction| {
        transaction.timestamp >= cmd.from_time
          && transaction.timestamp <= cmd.to_time
      });
    }

    // ! sort if need
    if cmd.sort_method.is_none() {
      return simple_trans;
    };
    let method = cmd.sort_method.unwrap();
    if method == String::from("date-asc") {
      simple_trans.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }
    if method == String::from("date-desc") {
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
// 用户点击导入钱包自动调用这个接口.存transacs 到后端
// #[update(guard = "user_owner_guard")]
#[update]
fn sync_transaction_record(
  cmd: Vec<SyncTransactionCommand>,
) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    for one_wallet in cmd {
      // FIXME fix already exsit transac . 检查最近一条的hash是否和db上的
      // hash一样.一样则返回 "transactions already newest. nothing append since
      // last time sync"
      // let latest_hash = one_wallet.history[0].hash.clone();
      // ! remove dup:

      // let hash_db=
      // ! append records
      for one_rec in one_wallet.history.clone() {
        ctx.id = ctx.id + 1;
        let id = ctx.id;
        let _ret = ctx.trans_f_srv.add_transaction_record(id, one_rec.clone());

        // ! copy transF to transB
        ctx.id = ctx.id + 1;
        let id2 = ctx.id;
        let trans_b = convert_trans_f_to_trans_b(one_rec, id2);
        let _ = ctx.wallet_transc_srv.add_transaction_impl(trans_b);
      }

      // ! update wallet info
      let mut wallet_profile = ctx
        .wallet_service
        .query_a_wallet(one_wallet.walletId)
        .expect("no such wallet");
      wallet_profile.last_sync_time = now();
      // FIXME this not work as semantics . still 0
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
    details: trans_f.details.clone(),
    memo: String::new(),
    address,
    tag: Vec::new(),
    manual: false,
    comment: String::new(),
  }
}

#[update(guard = "user_owner_guard")]
fn calculate_tax(
  wallets: Vec<WalletAddress>,
  method: String,
  exclued_tags: Vec<String>,
) -> String {
  CONTEXT.with(|ctx| {
    let mut ctx = ctx.borrow_mut();
    for one_wallet in wallets {
      let map = ctx.wallet_transc_srv.query_one_wallet_trans(one_wallet);
      let vec_data = map
        .values()
        .next()
        .expect("wallet transaction empty")
        .clone();

      // ! filter if got flag. air drop ...
      let filtered_vec_data: Vec<_> = vec_data
        .into_iter()
        .filter(|one| !one.tag.iter().any(|tag| exclued_tags.contains(tag)))
        .collect();

      // TODO ! cal profit using : lifo is for next milestone
      // /home/btwl/code/ic/tax_lint/backend/ohter_test/tax_test/main.rs
      // ! calculate base on method: fifo lifo.
      if method == "fifo".to_string() {
        for mut one in filtered_vec_data {
          if one.t_type == "RECEIVE".to_string() {
            one.details.profit = 0.0;
          } else if one.t_type == "SEND".to_string() {
            let profit = one.details.value - one.details.cost;
            one.details.profit = profit;
          }
          let _ = ctx.wallet_transc_srv.update_transaction_impl(one);
        }
      } else {
        return "only support fifo now".to_string();
      }
    }
    return "calculate success".to_string();
  })
}

struct Purchase {
  price: f64,
  amount: f64,
}

fn calculate_cost_fifo(transaction: TransactionB) -> f64 {
  let mut purchase_queue: VecDeque<Purchase> = VecDeque::new();

  if transaction.t_type == "RECEIVE" {
    let price = transaction.details.price;
    let amount = transaction.details.amount;
    purchase_queue.push_back(Purchase { price, amount });
    return 0.0;
  } else if transaction.t_type == "SEND" {
    let mut cost = 0.0;
    let mut send_amount = transaction.details.amount;

    while send_amount > 0.0 && !purchase_queue.is_empty() {
      let earliest_purchase = purchase_queue.pop_front().unwrap();

      if earliest_purchase.amount <= send_amount {
        cost += earliest_purchase.price * earliest_purchase.amount;
        send_amount -= earliest_purchase.amount;
      } else {
        cost += earliest_purchase.price * send_amount;
        purchase_queue.push_front(Purchase {
          price: earliest_purchase.price,
          amount: earliest_purchase.amount - send_amount,
        });
        send_amount = 0.0;
      }
    }

    return cost;
  } else {
    return 0.0;
  }
}

fn test1(){
  ic_cdk::api::time();
  ic_cdk::api::caller();
  // ic_cdk::api::
}