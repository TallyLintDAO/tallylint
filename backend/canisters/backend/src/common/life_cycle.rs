use std::io::{Read, Write};

use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::*;

use canister_tracing_macros::trace;

use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;

use crate::c_http::post::get_payload_from_dropbox;

use crate::CONTEXT;
use stable_memory::*;
#[init]
fn init() {
  ic_cdk::setup();
  let context = CanisterContext {
    env: Box::new(CanisterEnvironment {}),
    ..CanisterContext::default()
  };
  let _now = context.env.now();
  // let _creator1 = GOVERNANCE_BTWL.with(|g| *g);
  // let _creator2 = GOVERNANCE_ZHOU.with(|g| *g);

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
#[allow(dead_code)]
#[pre_upgrade]
#[trace]
fn pre_upgrade() {
  set_stable_mem_use_payload_simple();
}

#[allow(dead_code)]
#[post_upgrade]
#[trace]
fn post_upgrade() {
  let json = get_payload_from_stable_mem_simple();

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

#[query(guard = "admin_guard")]
pub fn collect_running_payload() -> String {
  let mut json: String = String::new();
  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id;
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_transc_srv.records.values().cloned());
    let transactions =
      Vec::from_iter(context.trans_f_srv.transactions.values().cloned());
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
      transactions,
    };

    // this call candid serialization. custom impl of serialization diff to
    // serde lib . ic_cdk::storage::stable_save((state, permit, users,
    // roles)).unwrap();

    // json = serde_json::to_string(&payload).unwrap();
    json = format!(r#"{}"#, serde_json::to_string(&payload).unwrap());
  });
  return json;
}

#[update(guard = "admin_guard")]
fn set_payload_using_stable_mem() {
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

/**
 * TEST OK
 */
#[update(guard = "admin_guard")]
fn set_stable_mem_use_payload() {
  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id;
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_transc_srv.records.values().cloned());
    let transactions =
      Vec::from_iter(context.trans_f_srv.transactions.values().cloned());
    let neurons =
      Vec::from_iter(context.neuron_service.neurons.values().cloned());
    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
      transactions,
    };

    let json = serde_json::to_string(&payload).unwrap();
    // ic_cdk::println!(
    //   "\x1b[31m SAVING THE PAYLOAD INTO STABLE STUCTURE: \x1b[0m  \n {}",
    //   json
    // );

    let mut memory = get_upgrades_memory();
    let mut writer = get_writer(&mut memory);
    let ret = writer.write_all(json.as_bytes());
    ret.expect("Failed to write to writer");
  });
}

/**
 * TEST OK!!! no trailing chars!
 */
#[update(guard = "admin_guard")]
fn set_stable_mem_use_payload_simple() {
  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id;
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_transc_srv.records.values().cloned());
    let transactions =
      Vec::from_iter(context.trans_f_srv.transactions.values().cloned());
    let neurons =
      Vec::from_iter(context.neuron_service.neurons.values().cloned());
    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
      transactions,
    };

    let json = serde_json::to_string(&payload).unwrap();
    // ic_cdk::println!(
    //   "\x1b[31m SAVING THE PAYLOAD INTO STABLE STUCTURE: \x1b[0m  \n {}",
    //   json
    // );

    // `(json,)`  is tuple syntax in rust ,creates a tuple with one element
    stable_save((json,)).expect("stable_save() faile");
  });
}

/**
 * TEST OK
 */
#[query(guard = "admin_guard")]
pub fn get_payload_from_stable_mem() -> String {
  let mut buf = Vec::new();
  let mem_0 = get_upgrades_memory();
  let mut reader = get_reader(&mem_0);
  reader
    .read_to_end(&mut buf)
    .expect("Failed to read from reader");
  let json = String::from_utf8_lossy(&buf);
  let json = json.trim_end_matches(char::from(0)); // Trim trailing zeros

  // ic_cdk::println!("\x1b[31m WHAT GET FROM stable mem:  \x1b[0m  {}", json);
  return json.to_string();
}

/**
 * TEST OK!!! no trailing chars!
 */
#[query(guard = "admin_guard")]
pub fn get_payload_from_stable_mem_simple() -> String {
  let (json,): (String,) =
    stable_restore().expect("failed to exec stable_restore()");

  // ic_cdk::println!("\x1b[31m WHAT GET FROM stable mem:  \x1b[0m  {}", json);
  return json.to_string();
}

#[update(guard = "admin_guard")]
pub async fn set_payload_using_dropbox(
  // get short-term token : https://www.dropbox.com/developers/apps/info/qi2656n62bhls4u
  token: String,
  // get from save_payload_to_dropbox fuction output
  date_time_version_tag: String,
) -> bool {
  let db_json = get_payload_from_dropbox(token, date_time_version_tag).await;

  // ic_cdk::println!("json: {}", db_json); // this print debug info to
  // ic-replica node console. TODO any possible to set log level and detect
  // dev-env or prod-env to optional log ?

  let payload: CanisterDB = serde_json::from_str(&db_json).unwrap();

  let stable_state = CanisterContext::from(payload);
  CONTEXT.with(|s| {
    let mut state = s.borrow_mut();
    *state = stable_state;
    return true;
  })
}

use ic_cdk::api::management_canister::http_request::{
  http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
// TODO
// now work ok. but cant use in pre_upgrade in ic canister.
// need to make this function blocking . cant be async.
// need to dive deeper in this programming.
/**
 * if src is 0 use running payload.
 * if NOT 0 use stable mem
 */
#[ic_cdk::update]
pub async fn save_payload_to_dropbox(
  token: String,
  src: u32,
  cycles: u128,
) -> String {
  let host = "content.dropboxapi.com";
  let url = "https://content.dropboxapi.com/2/files/upload";

  let timestamp = ic_cdk::api::time();
  use crate::tools::time_tool::timestamp_to_date;
  let time = timestamp_to_date(timestamp);

  let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}",token).to_string(),
        },

        HttpHeader {
        name: "Dropbox-API-Arg".to_string(),
        // path: dst to dropbox folder.
        value: format!("{{\"autorename\":false,\"mode\":\"add\",\"mute\":false,\"path\":\"/taxlint/payload_{}.json\",\"strict_conflict\":false}}", time),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            // value: "application/json".to_string(),
            value: "application/octet-stream".to_string(),
        },
    ];
  let json_string: String;
  if src == 0 {
    json_string = collect_running_payload();
  } else {
    json_string = get_payload_from_stable_mem();
  }

  let json_utf8: Vec<u8> = json_string.into_bytes();
  let request_body: Option<Vec<u8>> = Some(json_utf8);

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: request_body,
    transform: None, //optional for request
  };

  // use crate::calculate_cost;
  // let json_length = json_utf8.len() as u64;
  // let cycles = calculate_cost(16, json_length, 1000);

  match http_request(request, cycles).await {
    Ok((response,)) => {
      let str_body = String::from_utf8(response.body)
        .expect("Transformed response is not UTF-8 encoded.");
      // ic_cdk::api::print(format!("{:?}", str_body));

      let result: String = format!(
        "{}. See more info of the request sent at: {}/inspect \n timestamp as version number: {}",
        str_body, url,time
      );
      result
    }
    Err((r, m)) => {
      let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
      message
    }
  }
}

#[query(guard = "admin_guard")]
fn do_pre_upgrade_and_print_db() -> String {
  CONTEXT.with(|c| {
    let context = c.borrow();
    let id = context.id; // global increamenter.
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    let wallets =
      Vec::from_iter(context.wallet_service.wallets.values().cloned());
    let records =
      Vec::from_iter(context.wallet_transc_srv.records.values().cloned());
    let transactions =
      Vec::from_iter(context.trans_f_srv.transactions.values().cloned());
    let neurons =
      Vec::from_iter(context.neuron_service.neurons.values().cloned());

    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
      transactions,
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

use crate::common::guard::admin_guard;
// TODO not work as clean_db should do yet.
#[update(guard = "admin_guard")]
fn clean_db() -> bool {
  return false;
}

// #[cfg(test)]
// mod tests {
//   use crate::{
//     c_http::post::get_payload_from_dropbox, common::context::CanisterDB,
//   };

//   // use futures::executor::block_on;
//   use std::fs::File;
//   use std::io::Read;
//   #[test]
//   fn test_deserialize() {
//     let token=String::from("sl.BxPmJ_Y5qKXvWPtfPwon2tAIuGG-mkXQ0BT_c-13SAcN2Fv7jZOBpKKodcBHdULtrtC0OU7b1SUFQ5J0n-NcKOHNqa_D_Xoa-w2qwfq7U04c9rlqaPi_pzUpTQ2dy-3CL8RFB5KnKlr1-5cWxz0PddM");
//     let mut file =
//       File::open("/home/btwl/code/ic/tax_lint/backend/payload.json")
//         .expect("Unable to open the file");
//     let mut db_json = String::new();
//     file
//       .read_to_string(&mut db_json)
//       .expect("Unable to read the file");
//     eprintln!("{}", db_json);
//     let payload: CanisterDB = serde_json::from_str(&db_json).unwrap();
//     eprintln!("{}", payload.id);
//   }
// }
