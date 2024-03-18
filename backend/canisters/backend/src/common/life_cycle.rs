// TODO data flows 4 place:
// 1. canister memory(heap)
// 2. canister disk(stable memory)
// 3. admin terminal
// 4. dropbox


use ic_cdk::api::management_canister::http_request::{
  http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::*;

use canister_tracing_macros::trace;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;

use crate::common::guard::admin_guard;
use tracing::info;

use crate::c_http::post::{get_payload_from_dropbox, TERA};

use crate::CONTEXT;

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
#[pre_upgrade]
#[trace]
fn pre_upgrade() {
  set_stable_mem_using_payload();

  // TODO  !!!!should do this as backup everytime upgrade ic canister
  //  dfx canister call  backend do_pre_upgrade_and_print_db --network ic
  // TODO
  // ! also better save data to dropbox as backup better !
}

#[post_upgrade]
#[trace]
fn post_upgrade() {
  // THIS will not print msg. not call from `update` flag.
  // my_post_upgrade();
}

#[query(guard = "admin_guard")]
fn set_stable_mem_using_payload() {
  let json = collect_running_payload();
  stable_save((json.clone(),)).expect("stable_save() fail!!!!");
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
fn set_payload_using_stable_mem() -> String {
  let json = get_payload_from_stable_mem_simple();

  let ret = serde_json::from_str::<CanisterDB>(&json);
  match ret {
    Err(e) => {
      format!("!!!! deserialize_error: !!!! {:?}", e)
    }
    Ok(payload) => {
      let stable_state = CanisterContext::from(payload);
      CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = stable_state;
      });
      "upgrade_successful!".to_string()
    }
  }
}




#[update(guard = "admin_guard")]
pub async fn set_payload_using_dropbox(
  // get short-term token : https://www.dropbox.com/developers/apps/info/qi2656n62bhls4u
  token: String,
  // get from save_payload_to_dropbox fuction output
  date_time_version_tag: String,
) -> String {
  let json = get_payload_from_dropbox(token, date_time_version_tag).await;

  let ret = serde_json::from_str::<CanisterDB>(&json);
  match ret {
    Err(e) => {
      format!("!!!! deserialize_error: !!!! {:?}", e)
    }
    Ok(payload) => {
      let stable_state = CanisterContext::from(payload);
      CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = stable_state;
      });
      "upgrade_successful!".to_string()
    }
  }
}


// TODO
// now work ok. but cant use in pre_upgrade in ic canister.
// need to make this function blocking . cant be async.
// need to dive deeper in this programming.
/**
 * if src IS 0 use running payload.
 * if NOT 0 use stable mem
 */
#[ic_cdk::update]
pub async fn save_payload_to_dropbox(token: String, src: u32) -> String {
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
    json_string = get_payload_from_stable_mem_simple();
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

  let cycles = 1 * TERA;
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
    let payload = CanisterDB {
      id,
      users,
      wallets,
      records,
      neurons,
      transactions,
    };
    json = format!(r#"{}"#, serde_json::to_string(&payload).unwrap());
  });
  return json;
}

#[query(guard = "admin_guard")]
pub fn get_payload_from_stable_mem_simple() -> String {
  let (json,): (String,) =
    stable_restore().expect("failed to exec stable_restore()");
  return json;
}