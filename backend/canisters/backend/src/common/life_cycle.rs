use std::io::{Read, Write};
use std::str::FromStr;

use candid::Principal;
use ic_cdk_macros::*;

use canister_tracing_macros::trace;

use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;

use crate::common::constants::PROXY_CANISTER_ID;
use crate::{
  http_init, http_post_upgrade, CONTEXT, GOVERNANCE_BTWL, GOVERNANCE_ZHOU,
};
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

  http_init(Principal::from_str(PROXY_CANISTER_ID).unwrap());
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
// old version . last version exec.
#[pre_upgrade]
#[trace]
fn pre_upgrade() {
  // TODO try this.
  // storage::stable_save();
  // storage::stable_restore();

  // TODO
  // this log send to ic-os machine. maybe we can send log to a private web2
  // server ? or how do we check the *main-net* log on ic-os machine ? dfx
  // local replica seeing log OK. also dfx::print ok . :::: ANSWER: no way to
  // see main net ic_cdk print yet.
  info!("Pre-upgrade starting");
  ic_cdk::println!("Pre-upgrade starting");
  // use tokio::runtime::Runtime;
  // let rt = Runtime::new().unwrap();
  // rt.block_on(async {
  //   save_payload_to_dropbox();
  // });
  // ic_cdk::println!("run: save_payload_to_dropbox_blocking()");
  // save_payload_to_dropbox_blocking();
  CONTEXT.with(|c| {
    // collecting data
    let context = c.borrow();
    let id = context.id;
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

    // save to db
    let json = serde_json::to_string(&payload).unwrap();
    ic_cdk::println!(
      "\x1b[31m SAVING THE PAYLOAD INTO STABLE STUCTURE: \x1b[0m  \n {}",
      json
    );
    let mut memory = get_upgrades_memory();
    let mut writer = get_writer(&mut memory);
    let ret = writer.write_all(json.as_bytes());
    ret.expect("Failed to write to writer");
  });
}

#[allow(dead_code)]
// #[post_upgrade]
#[trace]
fn post_upgrade() {
  http_post_upgrade(Principal::from_str(PROXY_CANISTER_ID).unwrap());
  let mut buf = Vec::new();
  let memory = get_upgrades_memory();
  let mut reader = get_reader(&memory);
  reader
    .read_to_end(&mut buf)
    .expect("Failed to read from reader");
  let mut json = String::from_utf8_lossy(&buf).to_string();
  ic_cdk::println!("\x1b[31m WHAT GET FROM stable mem:  \x1b[0m  {}", json);

  let end_of_json = json.rfind('}').unwrap_or(0) + 1;
  json = json[..end_of_json].to_string();
  ic_cdk::println!(
    "\x1b[31m AFTER PROCESS of payload data:  \x1b[0m  {}",
    json
  );

  let ret = serde_json::from_str::<CanisterDB>(&json);
  let payload = match ret {
    Ok(value) => value,
    Err(e) => {
      ic_cdk::println!("!!!! deserialize error: !!!! {:?}", e);
      return;
    }
  };
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

#[query]
pub fn get_payload() -> String {
  let mut json: String = String::new();
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

    // json = serde_json::to_string(&payload).unwrap();
    json = format!(r#"{}"#, serde_json::to_string(&payload).unwrap());
  });
  return json;
}

#[update]
fn set_payload() {
  let memory = get_upgrades_memory();
  let mut reader = get_reader(&memory);

  let mut payload_json = String::new();
  reader
    .read_to_string(&mut payload_json)
    .expect("Failed to read from reader");

  // Handle trailing characters
  // TODO this maybe danger. is serialize format not good enough.
  // TODO should do data backup data to ic-VM-slot(2) ...

  // let end_of_json = payload_json.rfind('}').unwrap_or(0) + 1;
  // payload_json = payload_json[..end_of_json].to_string();

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

#[update]
/**
 * TEST OK
 */
fn set_stable_mem_use_payload() {
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
    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
    };

    let json = serde_json::to_string(&payload).unwrap();
    ic_cdk::println!(
      "\x1b[31m SAVING THE PAYLOAD INTO STABLE STUCTURE: \x1b[0m  \n {}",
      json
    );

    let mut memory = get_upgrades_memory();
    let mut writer = get_writer(&mut memory);
    let ret = writer.write_all(json.as_bytes());
    ret.expect("Failed to write to writer");
  });
}

/**
 * TEST OK
 */
#[query]
fn get_payload_from_stable_mem() {
  let mut buf = Vec::new();

  let memory = get_upgrades_memory();
  let mut reader = get_reader(&memory);
  reader
    .read_to_end(&mut buf)
    .expect("Failed to read from reader");
  let json = String::from_utf8_lossy(&buf);
  ic_cdk::println!("\x1b[31m WHAT GET FROM stable mem:  \x1b[0m  {}", json);
}
