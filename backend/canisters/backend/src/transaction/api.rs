use std::collections::HashMap;
use std::collections::VecDeque;

use ic_cdk_macros::{query, update};

use super::domain::*;
use super::service::WalletAddress;
use crate::common::context::{get_caller, now};
use crate::common::guard::admin_guard;
use crate::common::guard::user_owner_guard;
use crate::common::times::timestamp_ms_float_to_ns;
use crate::wallet::domain::HistoryQueryCommand;
use crate::wallet::service::WalletId;
use crate::{TransactionB, CONTEXT};

// TODO use: AddRecordCommand . front end dont need to input
// id . id gen by backend. TODO 测试 id 正常生成且不冲突
#[update(guard = "user_owner_guard")]
fn add_transaction(mut data: TransactionB) -> Result<WalletId, String> {
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
        // TODO update wallet.transactions numbers

        //           // ! update wallet info
        //   let mut wallet_profile = ctx
        //     .wallet_service
        //     .query_a_wallet(one_wallet.walletId)
        //     .expect("no such wallet");
        //   wallet_profile.last_sync_time = now();
        //   // FIXME this not work as semantics . still 0
        //   wallet_profile.transactions = one_wallet.history.len() as u64;
        //   wallet_profile.last_transaction_time = timestamp_ms_float_to_ns(
        //     one_wallet.history.get(0).unwrap().clone().timestamp,
        //   );
        //   ctx
        //     .wallet_service
        //     .update_wallet(wallet_profile, get_caller());
        // }
        return Ok(id);
      }
      Err(msg) => Err(msg),
    }
  })
}

#[update(guard = "user_owner_guard")]
fn delete_transaction(id: WalletId) -> Result<WalletId, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_transc_srv.delete_transaction_by_id_impl(id);
    match ret {
      Ok(_) => {
        // TODO update wallet.transactions numbers
        // TODO delete the id as fast bmap index into the wallet struct.
        Ok(id)
      }
      Err(msg) => Err(msg),
    }
  })
}

#[query(guard = "user_owner_guard")]
fn query_wallets_synced_transactions(
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
fn query_all_transactions() -> Result<HashMap<WalletId, TransactionB>, String> {
  CONTEXT.with(|c| {
    let ctx = c.borrow_mut();
    let rec = ctx.wallet_transc_srv.query_all_transactions();
    return Ok(rec);
  })
}

#[update(guard = "user_owner_guard")]
fn update_transaction(mut data: TransactionB) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();

    // bind addr
    let address = match data.t_type.as_str() {
      "SEND" => data.details.from.clone(),
      "RECEIVE" => data.details.to.clone(),
      _ => WalletAddress::default(), // You can handle other cases here
    };
    data.address = address;

    let ret = ctx.wallet_transc_srv.update_transaction_impl(data);
    match ret {
      Ok(_) => Ok(true),
      Err(msg) => Err(msg),
    }
  })
}

#[query(guard = "user_owner_guard")]
fn query_one_transaction(id: WalletId) -> Result<TransactionB, String> {
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
#[update(guard = "user_owner_guard")]
// #[update]
fn sync_transaction_record(
  cmd: Vec<SyncTransactionCommand>,
) -> Result<bool, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    for one_wallet in cmd {
      // FIXME fix already exsit transac . 检查最近一条的hash是否和db上的.
      // TODO WARN:remove 是临时方案.应该前端每次传增量数据.
      // 才不必要传过量数据,花费不合理 hash一样.一样则返回 "transactions
      // already newest. nothing append since last time sync"
      // let latest_hash = one_wallet.history[0].hash.clone();
      // ! remove current all.
      let w_addr = ctx.wallet_service.get_addr_by_id(one_wallet.walletId);
      ctx.trans_f_srv.delete_all_by_addr(w_addr.clone());
      ctx.wallet_transc_srv.delete_transaction_by_addr(&w_addr);

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
  id: WalletId,
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
      let tans_map = ctx.wallet_transc_srv.query_one_wallet_trans(one_wallet);
      let vec_data = tans_map
        .values()
        .next()
        .expect("wallet transaction empty")
        .clone();
      if vec_data.is_empty(){
        return "ERROR :NO TRANSACTIONS ! ".to_string();
      }

      // ! filter if got flag. air drop ...
      let filtered_vec_data: Vec<_> = vec_data
        .into_iter()
        .filter(|one| !one.tag.iter().any(|tag| exclued_tags.contains(tag)))
        .collect();
      if filtered_vec_data.is_empty(){
        return "ERROR :NO filtered TRANSACTIONS ! ".to_string();
      }


      // ! calculate base on method: fifo lifo.
      // get tax_transac for calculation
      let tax_transac: Vec<TransactionForTax> = filtered_vec_data
        .clone()
        .into_iter()
        .map(TransactionForTax::from)
        .collect();
      if tax_transac.is_empty(){
        return "ERROR :NO tax TRANSACTIONS ! ".to_string();
      }

      let taxed_vec_trans = calculate_gain_or_loss(tax_transac, method.clone());
      if taxed_vec_trans.is_empty(){
        return "ERROR tax calculation abort! no such calculate method ! ".to_string();
      }
      // map tax into transb_db
      let trans_b = map_taxTrans_to_transB(taxed_vec_trans, filtered_vec_data);
      if trans_b.is_empty(){
        return "ERROR :NO tax-calculated trans_b TRANSACTIONS ! ".to_string();
      }
      for one in trans_b {
        let _ = ctx.wallet_transc_srv.update_transaction_impl(one);
      }
    }
    return "calculate success!".to_string();
  })
}


#[query(guard = "user_owner_guard")]
pub fn greet_test()->String{
  ic_cdk::println!("got greet_test() call");
  return "hello agent!".to_string();
}
