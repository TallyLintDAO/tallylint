use std::borrow::Borrow;
use std::io::Read;

use ic_cdk_macros::*;

use canister_tracing_macros::trace;
use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;
use crate::{CONTEXT, GOVERNANCE_BTWL, GOVERNANCE_ZHOU};
use stable_memory::*;
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
// #[pre_upgrade]
#[trace]
// old version . last version exec.
fn pre_upgrade() {
  info!("Pre-upgrade starting");
  // let _logs = canister_logger::export_logs();
  // let _traces = canister_logger::export_traces();

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

    // TODO make this serialization one by one . just make sure every part is
    // serialize ok. and then combine them inro one bin file . or use
    // multiple VM on ic-node-replica FS.(ref stable structure on IC machine)
    // also the deserialization part one by one . users, then walletes, then
    // records... furthur: encasulate each serde of users,walletes, into a
    // generic serde function. even furthur. we can use multi-thread for
    // each. (when have many data to serde)

    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
    };

    // this call candid serialization. custom impl of serialization diff to
    // serde lib . ic_cdk::storage::stable_save((state, permit, users,
    // roles)).unwrap();

    // let json=serde_json::to_string_pretty(&payload).unwrap();
    // println!("{}", json);
    // save canister fs to ic-replica.
    let mut memory = get_upgrades_memory();
    let writer = get_writer(&mut memory);
    let ret = serializer::serialize(payload, writer);
    if ret.is_err() {
      info!("serialize err: {:?}", ret.err());
    } else {
      info!("serialize ok,old data saved to ic-fs.");
    }

    // IMPORTANT erase db in running canister.(ic or local)
    // dfx deploy backend  -m reinstall
  });
}

// #[post_upgrade]
#[trace]
fn post_upgrade() {
  // use reader  make the whole serde process become a Volcano/Pipeline Model
  // process procedure use string as a whole file is a Materialization Model
  // process procedure

  // IMPORTANT
  // load canister fs from ic-replica
  // () means retrieve multiple db. a collection of tuples
  let memory = get_upgrades_memory();
  let mut reader = get_reader(&memory);

  let mut payload_json = String::new();
  reader
    .read_to_string(&mut payload_json)
    .expect("Failed to read from reader");

  // Handle trailing characters
  // TODO this maybe danger. is serialize format not good enough.
  // TODO should do data backup data to ic-VM-slot(2) ...

  let end_of_json = payload_json.rfind('}').unwrap_or(0) + 1;
  payload_json = payload_json[..end_of_json].to_string();

  // TODO this way to fix deserialize err. type force casting here.
  // find the old version data structure. and then do deserialize. find old data
  // structure and then mannuly do it
  // let ret_json = serializer::deserialize(reader);
  // if ret_json.as_ref().is_err() {
  //   info!("deserialize err: {:?}", ret_json.as_ref().err());
  //   println!("deserialize err: {:?}", ret_json.as_ref().err());

  //   // Handle trailing characters
  //   // let end_of_json = ret_json.rfind('}').unwrap_or(0) + 1;
  //   // ret_json = ret_json[..end_of_json].to_string();
  // }

  // somehow when serialized add this kind of stuff.
  // TODO . need to print the serialized data to programmer check it .
  //     let trailing_json = format!("{} extra characters", json); // add
  // trailing characters

  let payload: CanisterDB = serde_json::from_str(&payload_json).unwrap();
  info!("deserialize ok,old data loaded from ic-fs.");

  // this deserialize procedure is open an ic replica node local file . and then
  // use a reader to read its local disk file and then load it into memory.
  // and deserialize it .

  let stable_state = CanisterContext::from(payload);
  CONTEXT.with(|s| {
    let mut state = s.borrow_mut();
    *state = stable_state;
  });
}

// the whole update canister procedure: ( on a IC node program running on a
// ubuntu server.) 1.(ser DB)save current thread local datas in to a binary
// file. 2.(update code logic)replace .wasm file with new .wasm file
// 3.(de DB) load TL data into memory with that running .wasm file process addr
// space.

// TODO : in step1 how to save the DB to dev machine?(contains user data and
// running logs) and (the reverse step:) how dev machine upload DB file to IC
// node machine ?
