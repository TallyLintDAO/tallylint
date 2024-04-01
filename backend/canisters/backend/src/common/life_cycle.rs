// TODO data flows 4 place:
// 1. canister memory(heap)
// 2. canister disk(stable memory)
// 3. admin dev machine

// TODO update ic process:
// TOOL: use rs_agent lib codes:
// 1. run  set_stable_mem_using_payload_simple() downlaod db to dev machine.
//    give time tag
// 2. run deploy ic
// 3. put db file into ic canister

pub const TERA: Cycles = 1_000_000_000_000;
pub type Cycles = u128;

use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::*;

use super::context::CanisterContext;

use crate::common::guard::admin_guard;

use crate::STATE;
#[init]
fn init() {
  ic_cdk::setup();
  // info!("canister initialization complete");
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
fn pre_upgrade() {
  // set_stable_mem_using_payload();
  // TODO  !!!!should do this as backup everytime upgrade ic canister
  //  dfx canister call  backend do_pre_upgrade_and_print_db --network ic
  // TODO
  // ! also better save data to dropbox as backup better !
}

#[post_upgrade]
fn post_upgrade() {
  // THIS will not print msg. not call from `update` flag.
  // my_post_upgrade();
}

#[update(guard = "admin_guard")]
fn set_stable_mem_using_payload_simple() {
  let json = collect_running_payload_simple();
  stable_save((json.clone(),)).expect("stable_save() fail!!!!");
}

#[update(guard = "admin_guard")]
fn set_stable_mem_using_payload_simple_raw() {
  STATE.with(|ctx| {
    stable_save((ctx,)).expect("stable_save() fail!!!!");
  });
}

// the whole update canister procedure: ( on a IC node program running on a
// ubuntu server.) 1.(ser DB)save current thread local datas in to a binary
// file. 2.(update code logic)replace .wasm file with new .wasm file
// 3.(de DB) load TL data into memory with that running .wasm file process addr
// space.

/**
 * ! This must exec everytime update ic net
 * IMPORTANT step. often causing deserialize_error!!!!
 */
#[update(guard = "admin_guard")]
fn set_payload_using_stable_mem_simple() -> String {
  let db_json = get_payload_from_stable_mem_simple();

  let ret = serde_json::from_str::<CanisterContext>(&db_json);
  match ret {
    Err(e) => {
      format!("!!!! deserialize_error: !!!! {:?}", e)
    }
    Ok(db_json) => {
      STATE.with(|s| {
        let mut state = s.borrow_mut();
        *state = db_json;
      });
      "upgrade_successful!".to_string()
    }
  }
}

#[update(guard = "admin_guard")]
fn set_payload_using_stable_mem_simple_raw() {
  STATE.with(|s| {
    let ctx = get_payload_from_stable_mem_simple_raw();
    let mut state = s.borrow_mut();
    *state = ctx;
  });
}

#[query(guard = "admin_guard")]
pub fn get_payload_from_stable_mem_simple() -> String {
  let (db_json,): (String,) =
    stable_restore().expect("failed to exec stable_restore()");
  return db_json;
}

#[query(guard = "admin_guard")]
pub fn get_payload_from_stable_mem_simple_raw() -> CanisterContext {
  let (raw_ctx,): (CanisterContext,) =
    stable_restore().expect("failed to exec stable_restore()");
  return raw_ctx;
}

// ! Important API to upload a file to canister
#[update(guard = "admin_guard")]
pub fn set_payload_using_dev_machine_file(payload: String) -> String {
  let db_json = payload;

  let ret = serde_json::from_str::<CanisterContext>(&db_json);
  match ret {
    Err(e) => {
      let ret = format!("!!!! deserialize_error: !!!! {:?}", e);
      return ret;
    }
    Ok(db_json) => {
      STATE.with(|s| {
        let mut state = s.borrow_mut();
        *state = db_json;
      });
      let ret = "####Congrates!: upgrade_successful!".to_string();
      return ret;
    }
  }
}

#[query(guard = "admin_guard")]
pub fn collect_running_payload_simple() -> String {
  let mut json: String = String::new();
  STATE.with(|ctx| {
    json = serde_json::to_string(&ctx).unwrap();
  });
  return json;
}

#[query(guard = "admin_guard")]
pub fn collect_running_payload_simple_raw() -> String {
  let mut json: String = String::new();
  STATE.with(|ctx| {
    json = format!(r#"{}"#, serde_json::to_string(&ctx).unwrap());
  });
  return json;
}

// TODO not work as clean_db should do yet.
#[update(guard = "admin_guard")]
fn clean_db() -> bool {
  return false;
}

#[cfg(test)]
mod tests {
  use crate::common::context::CanisterContext;

  // use futures::executor::block_on;
  use std::fs::File;
  use std::io::Read;
  #[test]
  fn test_deserialize() {
    let db_json = read_db_to_string_from_local_json_file(
      "/home/btwl/code/ic/tax_lint/backend/i_test/pl_01_no_newline.json"
        .to_owned(),
    );
    let payload_result: Result<CanisterContext, _> =
      serde_json::from_str(&db_json);
    match payload_result {
      Ok(payload) => eprintln!("PLID is :{}", payload.id),
      Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
  }

  fn read_db_to_string_from_local_json_file(f_path: String) -> String {
    let mut file = File::open(f_path).expect("Unable to open the file");
    let mut db_json = String::new();
    file
      .read_to_string(&mut db_json)
      .expect("Unable to read the file");
    db_json
  }

  #[test]
  fn test_deserialize_simple() {
    let ctx: CanisterContext = CanisterContext::new();
    let db_json = serde_json::to_string(&ctx).expect("serialize_err");

    let payload_result: Result<CanisterContext, _> =
      serde_json::from_str(&db_json);
    match payload_result {
      Ok(payload) => eprintln!("PLID is :{}", payload.id),
      Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
  }

  #[test]
  fn test_deserialize_complex() {
    let db_json = "xx".to_string();
    let payload_result: Result<CanisterContext, _> =
      serde_json::from_str(&db_json);
    match payload_result {
      Ok(payload) => eprintln!("PLID is :{}", payload.id),
      Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
  }

  /**
   * TEST OK
   */
  #[test]
  fn new_struct_deserial() {
    let db_json = read_db_to_string_from_local_json_file(
      "/home/btwl/code/ic/tax_lint/backend/i_test/new_ctx_struct_all_ic_data.json".to_owned(),
    );

    let payload_result: Result<CanisterContext, _> =
      serde_json::from_str(&db_json);
    match payload_result {
      Ok(payload) => eprintln!("PLID is :{}", payload.id),
      Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
  }
}
