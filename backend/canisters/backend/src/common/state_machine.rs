use ic_cdk::storage;
use ic_cdk_macros::*;

use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use crate::{CONTEXT, GOVERNANCE_BTWL, GOVERNANCE_ZHOU};

#[init]
fn init() {
  ic_cdk::setup();
  let context = CanisterContext {
    env: Box::new(CanisterEnvironment {}),
    ..CanisterContext::default()
  };
  let _now = context.env.now();
  let _creator1 = GOVERNANCE_BTWL.with(|g| *g);
  let _creator2 = GOVERNANCE_ZHOU.with(|g| *g);

  info!( "canister initialization complete");
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
fn pre_upgrade() {
  // with is a function can receive a function as para.
  // and | | syntax here means a function with no name.

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
    storage::stable_save((payload,)).expect("failed to save state data");
    // IMPORTANT erase db in running canister.(ic or local)
    // dfx deploy backend  -m reinstall
  });
}

#[post_upgrade]
fn post_upgrade() {
  // IMPORTANT
  // load canister fs from ic-replica
  // () means retrieve multiple db. a collection of tuples
  let (payload,): (CanisterDB,) =
    storage::stable_restore().expect("failed to restore users");
  let stable_state = CanisterContext::from(payload);
  CONTEXT.with(|s| {
    let mut state = s.borrow_mut();
    *state = stable_state;
  });
}
