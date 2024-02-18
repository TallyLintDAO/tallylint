
use ic_cdk_macros::*;

use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;
use crate::{CONTEXT, GOVERNANCE_BTWL, GOVERNANCE_ZHOU};

use stable_memory::*;

#[allow(dead_code)]
fn init() {
  ic_cdk::setup();
  let context = CanisterContext {
    env: Box::new(CanisterEnvironment {}),
    ..CanisterContext::default()
  };
  let _now = context.env.now();
  let _creator1 = GOVERNANCE_BTWL.with(|g| *g);
  let _creator2 = GOVERNANCE_ZHOU.with(|g| *g);

  info!("canister initialization complete");
}

#[query]
fn do_pre_upgrade_and_print_db() -> String {
  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id; // global increamenter.
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_record_service.records.values().cloned());
    let neurons =
      Vec::from_iter(context.neuron_service.neurons.values().cloned());

    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
    };

    let mut memory = get_upgrades_memory();
    {
      let writer = get_writer(&mut memory);
      let ret = serializer::serialize(payload.clone(), writer);
      if ret.is_err() {
        info!("serialize err: {:?}", ret.err());
      } else {
        info!("serialize ok,old data saved to ic-fs.");
      }
    }
    {
      let _reader = get_reader(&mut memory);
    }

    let json = serde_json::to_string(&payload).unwrap();

    ic_cdk::println!("json: {}", json); // this print debug info to ic-replica node console.
    return json;
  })
}

// TODO not work as clean_db should do yet.
#[update]
fn clean_db() -> String {
  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id;
    let users = Vec::new();
    let wallets = Vec::new();
    let records = Vec::new();
    let neurons = Vec::new();
    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
    };

    let mut memory = get_upgrades_memory();
    let writer = get_writer(&mut memory);
    let ret = serializer::serialize(payload.clone(), writer);
    if ret.is_err() {
      info!("serialize err: {:?}", ret.err());
    } else {
      info!("serialize ok,old data saved to ic-fs.");
    }

    let json = serde_json::to_string(&payload).unwrap();
    return json;
  })
}

