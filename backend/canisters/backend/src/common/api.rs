use std::borrow::Borrow;
use std::io::Read;

use ic_cdk_macros::*;

use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;
use crate::c_http::post::get_payload_from_dropbox;
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

/*
    if no db_json input as first para. get db from ic-fs.
*/
#[update]
pub async fn do_post_upgrade(token:String,date_time_version_tag: String) -> bool {
  // use reader  make the whole serde process become a Volcano/Pipeline Model
  // process procedure use string as a whole file is a Materialization Model
  // process procedure

  // IMPORTANT
  // load canister fs from ic-replica
  // () means retrieve multiple db. a collection of tuples
  let db_json = get_payload_from_dropbox(token,date_time_version_tag).await;
  ic_cdk::println!("json: {}", db_json); // this print debug info to ic-replica node console.

  // if db_json.len() == 0 {
  //   let memory = get_upgrades_memory();
  //   let mut reader = get_reader(&memory);
  //   reader
  //     .read_to_string(&mut db_json)
  //     .expect("Failed to read from reader");
  // }

  // Handle trailing characters
  // TODO this maybe danger. is serialize format not good enough.
  // TODO should do data backup data to ic-VM-slot(2) ...

  // let end_of_json = db_json.rfind('}').unwrap_or(0) + 1;
  // db_json = db_json[..end_of_json].to_string();

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

  ic_cdk::println!("json: {}", db_json); // this print debug info to ic-replica node console.
  let payload: CanisterDB = serde_json::from_str(&db_json).unwrap();

  info!("deserialize ok,old data loaded from ic-fs.");

  // this deserialize procedure is open an ic replica node local file . and then
  // use a reader to read its local disk file and then load it into memory.
  // and deserialize it .

  let stable_state = CanisterContext::from(payload);
  CONTEXT.with(|s| {
    let mut state = s.borrow_mut();
    *state = stable_state;
    return true;
  })
}

#[update]
pub async fn restore_db_from_dropbox(token:String,timestamp_as_version_tag: String) -> bool {
  // use reader  make the whole serde process become a Volcano/Pipeline Model
  // process procedure use string as a whole file is a Materialization Model
  // process procedure

  // IMPORTANT
  // load canister fs from ic-replica
  // () means retrieve multiple db. a collection of tuples
  let mut db_json = get_payload_from_dropbox(token,timestamp_as_version_tag).await;
  if db_json.len() == 0 {
    let memory = get_upgrades_memory();
    let mut reader = get_reader(&memory);
    reader
      .read_to_string(&mut db_json)
      .expect("Failed to read from reader");
  }

  // Handle trailing characters
  // TODO this maybe danger. is serialize format not good enough.
  // TODO should do data backup data to ic-VM-slot(2) ...

  let end_of_json = db_json.rfind('}').unwrap_or(0) + 1;
  db_json = db_json[..end_of_json].to_string();

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

  let payload: CanisterDB = serde_json::from_str(&db_json).unwrap();
  info!("deserialize ok,old data loaded from ic-fs.");

  // this deserialize procedure is open an ic replica node local file . and then
  // use a reader to read its local disk file and then load it into memory.
  // and deserialize it .

  let stable_state = CanisterContext::from(payload);
  CONTEXT.with(|s| {
    let mut state = s.borrow_mut();
    *state = stable_state;
    return true;
  })
}

// the whole update canister procedure: ( on a IC node program running on a
// ubuntu server.) 1.(ser DB)save current thread local datas in to a binary
// file. 2.(update code logic)replace .wasm file with new .wasm file
// 3.(de DB) load TL data into memory with that running .wasm file process addr
// space.

// TODO : in step1 how to save the DB to dev machine?(contains user data and
// running logs) and (the reverse step:) how dev machine upload DB file to IC
// node machine ?
