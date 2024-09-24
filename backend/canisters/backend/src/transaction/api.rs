use std::collections::HashMap;

use ic_cdk::caller;
use ic_cdk_macros::{query, update};

use super::domain::*;
use super::service::WalletAddress;
use crate::common::context::{get_caller, now, TimeStamp};
use crate::common::guard::admin_guard;
use crate::common::guard::user_owner_guard;
use crate::common::times::{
  timestamp_ms_float_to_ms_u64, timestamp_ms_float_to_ns,
};
use crate::wallet::domain::HistoryQueryCommand;
use crate::wallet::service::WalletId;
use crate::STATE;
use crate::{MySummary, TransactionB};

//前端必须指定加到哪个钱包.如果不加到任何钱包就是用户的taxlint专有钱包
#[update(guard = "user_owner_guard")]
fn add_transaction(mut data: TransactionB) -> Result<u64, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let id = ctx.id.clone();
    let result = ctx.wallet_transc_srv.add_transaction_impl(data.clone(), id);
    match result {
      Ok(_) => {
        // update wallet info
        // let mut cur_wallet = ctx.wallet_service.get_by_addr(data.address);
        // cur_wallet.transactions = cur_wallet.transactions + 1;
        // ctx.wallet_service.update_wallet(cur_wallet, caller());
        return Ok(result.unwrap());
      }
      Err(msg) => {
        return Err(msg);
      }
    }
  })
}
//delete transaction by wallet id
#[update(guard = "user_owner_guard")]
fn delete_transaction(wid: WalletId) -> Result<WalletId, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_transc_srv.delete_transactions_by_wid(wid);
    match ret {
      Ok(_) => Ok(wid),
      Err(msg) => Err(msg),
    }
  })
}
//根query transaction by id
#[query(guard = "user_owner_guard")]
fn query_one_transaction(id: TransactionId) -> Result<TransactionB, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let ret = ctx.wallet_transc_srv.query_one(id);
    match ret {
      Ok(t) => Ok(t),
      Err(msg) => Err(msg),
    }
  })
}
//query transactions that have been syncronised
#[query(guard = "user_owner_guard")]
fn query_synced_transactions(
  cmd: HistoryQueryCommand,
) -> Result<Vec<TransactionB>, String> {
  STATE.with(|c| {
    let ctx = c.borrow();
    let synced_transactions =
      ctx.wallet_transc_srv.query_synced_transactions(cmd.clone());
    Ok(synced_transactions)
  })
}
//query all transactions
#[query(guard = "admin_guard")]
fn query_all_transactions() -> Result<HashMap<WalletId, TransactionB>, String> {
  STATE.with(|c| {
    let ctx = c.borrow_mut();
    let rec = ctx.wallet_transc_srv.query_all_transactions();
    return Ok(rec);
  })
}
//update a transaction
#[update(guard = "user_owner_guard")]
fn update_transaction(mut data: TransactionB) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();

    // bind addr
    let address = match data.t_type.as_str() {
      "SEND" => data.details.from.clone(),
      "RECEIVE" => data.details.to.clone(),
      _ => WalletAddress::default(), //should never goes here
    };
    data.address = address;

    let ret = ctx.wallet_transc_srv.update_transaction_impl(data);
    match ret {
      Ok(_) => Ok(true),
      Err(msg) => Err(msg),
    }
  })
}
//
#[update(guard = "user_owner_guard")]
fn update_transaction_tag(id: u64, tag: String) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let one_trans = ctx.wallet_transc_srv.query_one(id);
    if one_trans.is_ok() {
      let mut one = one_trans.unwrap();
      one.tag = Some(tag);
      ctx
        .wallet_transc_srv
        .update_transaction_impl(one)
        .expect("update err");
      return Ok(true);
    } else {
      return Err("no such transaction".to_string());
    }
  })
}

#[update(guard = "user_owner_guard")]
fn remove_transaction_tag(id: u64) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let one_trans = ctx.wallet_transc_srv.query_one(id);
    if one_trans.is_ok() {
      let mut one = one_trans.unwrap();
      one.tag = None;
      ctx
        .wallet_transc_srv
        .update_transaction_impl(one)
        .expect("update err");
      return Ok(true);
    } else {
      return Err("no such transaction".to_string());
    }
  })
}

#[update(guard = "user_owner_guard")]
fn sync_transaction_record(
  data: Vec<SyncTransactionCommand>,
) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    for each_wallet in data {
      // ! append records and check if there is any duplicate
      for one_rec in each_wallet.history.clone() {
        // copy transF to transB
        let id = ctx.id;
        let trans_b = convert_trans_f_to_trans_b(one_rec, id);
        //save to wallet_transc_srv
        let result = ctx.wallet_transc_srv.add_transaction_impl(trans_b, id);
        match result {
          Ok(_) => {
            return Ok(true);
          }
          Err(msg) => {
            return Err(msg);
          }
        }
      }
      // ! update wallet info
      let mut wallet_profile = ctx
        .wallet_service
        .query_wallet_by_id(each_wallet.walletId)
        .expect("no such wallet");
      // FIXME use ms u64 .
      wallet_profile.last_sync_time = now();
      wallet_profile.transactions = each_wallet.history.len() as u64;
      wallet_profile.last_transaction_time = timestamp_ms_float_to_ns(
        each_wallet.history.get(0).unwrap().clone().timestamp,
      );
      ctx
        .wallet_service
        .update_wallet(wallet_profile, get_caller());
    }

    Ok(true)
  })
}

// Nedd set para in user config . cal_method and exclude_tags
#[update(guard = "user_owner_guard")]
fn calculate_tax() -> String {
  STATE.with(|ctx| {
    let mut ctx = ctx.borrow_mut();
    let wallets = ctx.wallet_service.get_all_addr_by_user(caller());
    //TODO 如果有panic的情况该如何处理
    // let exclude_tags = ctx.user_service.get_config(&caller()).unwrap().exclude_tags;

    for one_wallet in wallets {
      let tans_map = ctx.wallet_transc_srv.query_one_wallet_trans(one_wallet);
      let vec_data = tans_map
        .values()
        .next()
        .expect("wallet transaction empty")
        .clone();
      if vec_data.is_empty() {
        return "ERROR :NO TRANSACTIONS ! ".to_string();
      }

      // ! filter if got flag. air drop ...
      // let filtered_vec_data: Vec<TransactionB>;
      // if !exclude_tags.is_empty() {
      //   filtered_vec_data = vec_data
      //     .into_iter()
      //     .filter(|one| !one.tag.iter().any(|tag| exclude_tags.contains(tag)))
      //     .collect();
      //   if filtered_vec_data.is_empty() {
      //     return "ERROR :NO filtered TRANSACTIONS ! ".to_string();
      //   }
      // } else {
      //   filtered_vec_data = vec_data;
      // }

      // ! calculate base on method: fifo lifo.
      // get tax_transac for calculation
      //replace filtered_vec_datato vec_data
      let tax_transac: Vec<TransactionForTax> = vec_data
        .clone()
        .into_iter()
        .map(TransactionForTax::from)
        .collect();
      if tax_transac.is_empty() {
        return "ERROR :NO tax TRANSACTIONS ! ".to_string();
      }
      //TODO 如果有panic的情况该如何处理
      let method = ctx.user_service.get_config(&caller()).unwrap().tax_method;
      let taxed_vec_trans = calculate_gain_or_loss(tax_transac, method.clone());
      if taxed_vec_trans.is_empty() {
        return "ERROR tax calculation abort! no such calculate method ! "
          .to_string();
      }
      // map tax into transb_db
      //replace filtered_vec_datato vec_data
      let trans_b = map_taxTrans_to_transB(taxed_vec_trans, vec_data);
      if trans_b.is_empty() {
        return "ERROR :NO tax-calculated trans_b TRANSACTIONS ! ".to_string();
      }
      for one in trans_b {
        let _ = ctx.wallet_transc_srv.update_transaction_impl(one);
      }
    }
    return "calculate success!".to_string();
  })
}

// if start or end is 0. calculate all trans.
#[update(guard = "user_owner_guard")]
fn my_summary(start: TimeStamp, end: TimeStamp) -> Result<MySummary, String> {
  STATE.with(|ctx| {
    // donation loan_fee margin_fee tax loan_payment
    // margin_payment realted_PL gift lost
    let mut my_summary = MySummary {
      capital_gain_or_loss: 0.0,
      other_gain: 0.0, // TODO  final stage work
      income: 0.0,
      costs_expenses: 0.0, // TODO  final stage work
      gifts_dotations_lost_coins: 0.0,
    };
    let mut ctx = ctx.borrow_mut();
    let wallets = ctx.wallet_service.get_all_addr_by_user(caller());
    let mut all_trans = Vec::new();
    // get all trans by wallet addr into a vec and iter to sum all get profit.
    for one_wallet in wallets {
      let mut vec_trans = ctx
        .wallet_transc_srv
        .query_one_wallet_trans(one_wallet)
        .values()
        .next()
        .expect("wallet transaction empty")
        .clone();
      if vec_trans.is_empty() {
        return Err("NO TRANSACTIONS!".to_string());
      }
      all_trans.append(&mut vec_trans);
    }

    // filter by year range
    let filtered_trans: Vec<TransactionB>;
    if start == 0 || end == 0 {
      filtered_trans = all_trans;
    } else {
      filtered_trans = all_trans
        .into_iter()
        .filter(|trans| trans.timestamp >= start && trans.timestamp <= end)
        .collect();
      if filtered_trans.is_empty() {
        return Err(
          "year filter empty! no transaction this year or timestamp input err"
            .to_string(),
        );
      }
    };

    // cal summary
    for data in filtered_trans {
      if data.tag.is_none() {
        my_summary.capital_gain_or_loss =
          my_summary.capital_gain_or_loss + data.details.profit;
      } else if data.tag == Some("Airdrop".to_string()) {
        my_summary.income += data.details.amount * data.details.price;
      } else if data.tag == Some("Gift".to_string())
        || data.tag == Some("Donation".to_string())
        || data.tag == Some("Lost".to_string())
      {
        my_summary.gifts_dotations_lost_coins +=
          data.details.amount * data.details.price;
      }
    }
    return Ok(my_summary);
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
    wid: trans_f.wid,
    hash: trans_f.hash,
    timestamp: timestamp_ms_float_to_ms_u64(trans_f.timestamp),
    t_type: trans_f.t_type,
    details: trans_f.details.clone(),
    memo: String::new(),
    address,
    tag: None,
    manual: false,
    comment: String::new(),
  }
}

#[query(guard = "user_owner_guard")]
pub fn greet_test_agent() -> String {
  ic_cdk::println!("got greet_test() call");
  return "hello agent!".to_string();
}

// The test module
#[cfg(test)]
mod tests {

  // Test case functions
  #[test]
  fn time_test() {
    let str = "ok11111111111111111111111".to_string();
    if 1735660799_001u64 >= 1735660799_000u64
      && 1704038400_000u64 <= 1735660799_001u64
    {
      eprintln!("{}", str);
      assert!(true);
    } else {
      assert!(false);
    }
  }
}
