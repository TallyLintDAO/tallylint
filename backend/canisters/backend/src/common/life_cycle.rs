use ic_cdk::storage;
use ic_cdk_macros::*;

use canister_tracing_macros::trace;
use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;
use crate::{CONTEXT, GOVERNANCE_BTWL, GOVERNANCE_ZHOU};
  use stable_memory::*;
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

/**
 * 1. each time upgrade(such as CLI-cmd : dfx deploy ),
 * will *erase* all ic-DB (canister stable memory)
 * so we can:
 *      1.manually erase all,
 *      2.or , restore from a in memory data.(such as a
 * hashmap)ca
 * 2. transactional upgrade:
 * if pre_upgrade, upgrade ,post_upgrade
 * any step go wrong.
 * will revert to last version.
 */
// #[pre_upgrade] is a hook. everytime update canister will auto call this.
#[pre_upgrade]
#[trace]
// old version . last version exec.
fn pre_upgrade() {
  info!("Pre-upgrade starting");
  let _logs = canister_logger::export_logs();
  let _traces = canister_logger::export_traces();

  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id;
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_record_service.records.values().cloned());
    let neurons =
      Vec::from_iter(context.neuron_service.neurons.values().cloned());
    // Vec::new();
    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
    };
    // save canister fs to ic-replica.
    let mut memory = get_upgrades_memory();
    let writer = get_writer(&mut memory);
    serializer::serialize(payload, writer).unwrap();
    // IMPORTANT erase db in running canister.(ic or local)
    // dfx deploy backend  -m reinstall
  });
}

#[post_upgrade]
#[trace]
fn post_upgrade() {
  // IMPORTANT
  // load canister fs from ic-replica
  // () means retrieve multiple db. a collection of tuples
  let memory = get_upgrades_memory();


  let reader = get_reader(&memory);
  // TODO this way to fix deserialize err.
  // find the old version data structure. and then do deserialize. find old data structure and then mannuly do it 
  let (payload,): (CanisterDB,) = serializer::deserialize(reader).unwrap();

  let stable_state = CanisterContext::from(payload);
  CONTEXT.with(|s| {
    let mut state = s.borrow_mut();
    *state = stable_state;
  });
}
