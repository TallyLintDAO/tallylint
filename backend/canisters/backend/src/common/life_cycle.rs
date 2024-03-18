// TODO data flows 4 place:
// 1. canister memory(heap)
// 2. canister disk(stable memory)
// 3. admin terminal
// 4. dropbox

// 1. IMPORT IC MANAGEMENT CANISTER
//This includes all methods and types needed

use serde::{Deserialize, Serialize};

pub const TERA: Cycles = 1_000_000_000_000;
pub type Cycles = u128;

use ic_cdk::api::management_canister::http_request::{
  http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::*;

use canister_tracing_macros::trace;

use super::context::{CanisterContext, CanisterDB};

use crate::common::guard::admin_guard;
use tracing::info;

use crate::CONTEXT;

#[init]
fn init() {
  ic_cdk::setup();
  // let context = CanisterContext {
  //   env: Box::new(CanisterEnvironment {}),
  //   ..CanisterContext::default()
  // };
  // let _now = context.env.now();
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

#[update(guard = "admin_guard")]
fn set_stable_mem_using_payload() {
  let json = collect_running_payload();
  stable_save((json.clone(),)).expect("stable_save() fail!!!!");
}

#[update(guard = "admin_guard")]
fn set_stable_mem_using_payload_simple() {
  CONTEXT.with(|ctx| {
    let db_json = serde_json::to_string(ctx).expect("serialize_err");
    stable_save((db_json,)).expect("stable_save() fail!!!!");
  });
}

#[update(guard = "admin_guard")]
fn set_stable_mem_using_payload_simple_raw() {
  CONTEXT.with(|ctx| {
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
fn set_payload_using_stable_mem_simple() -> String {
  let db_json = get_payload_from_stable_mem_simple();

  let ret = serde_json::from_str::<CanisterContext>(&db_json);
  match ret {
    Err(e) => {
      format!("!!!! deserialize_error: !!!! {:?}", e)
    }
    Ok(db_json) => {
      CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = db_json;
      });
      "upgrade_successful!".to_string()
    }
  }
}

#[update(guard = "admin_guard")]
fn set_payload_using_stable_mem_simple_raw() {
  CONTEXT.with(|s| {
    let ctx = get_payload_from_stable_mem_simple_raw();
    let mut state = s.borrow_mut();
    *state = ctx;
  });
}

// !works ok in single node. multi node replica response not match
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
#[update(guard = "admin_guard")]
pub async fn set_payload_using_dropbox_simple(
  // get short-term token : https://www.dropbox.com/developers/apps/info/qi2656n62bhls4u
  token: String,
  // get from save_payload_to_dropbox fuction output
  date_time_version_tag: String,
) -> String {
  let db_json = get_payload_from_dropbox(token, date_time_version_tag).await;
  let ret = serde_json::from_str::<CanisterContext>(&db_json);
  match ret {
    Err(e) => {
      format!("!!!! deserialize_error: !!!! {:?}", e)
    }
    Ok(db_json) => {
      CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = db_json;
      });
      "upgrade_successful!".to_string()
    }
  }
}

#[update(guard = "admin_guard")]
pub async fn set_payload_using_dropbox_simple_raw(
  // get short-term token : https://www.dropbox.com/developers/apps/info/qi2656n62bhls4u
  token: String,
  // get from save_payload_to_dropbox fuction output
  date_time_version_tag: String,
) -> String {
  let db_json_u8: Vec<u8> =
    get_payload_from_dropbox_u8(token, date_time_version_tag)
      .await
      .unwrap();
  let db_json = String::from_utf8(db_json_u8)
    .expect("Transformed response is not UTF-8 encoded.");
  ic_cdk::println!("{}", db_json);

  let ret = serde_json::from_str::<CanisterContext>(&db_json);
  match ret {
    Err(e) => {
      format!("!!!! deserialize_error: !!!! {:?}", e)
    }
    Ok(db_json) => {
      CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = db_json;
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
pub fn collect_running_payload_simple() -> String {
  let mut json: String = String::new();
  CONTEXT.with(|ctx| {
    json = format!(r#"{}"#, serde_json::to_string(&ctx).unwrap());
  });
  return json;
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

#[update(guard = "admin_guard")]
pub fn send_payload_string_to_canister(payload: String) -> String {
  let data = payload;
  return data;
}

// !only works ok in single node. multi node replica response not match
#[ic_cdk::update]
pub async fn get_payload_from_dropbox(
  token: String,
  timestamp: String,
) -> String {
  let host = "content.dropboxapi.com";
  let url = "https://content.dropboxapi.com/2/files/download";

  let request_headers = vec![
    HttpHeader {
      name: "Host".to_string(),
      value: format!("{host}:443"),
    },
    HttpHeader {
      name: "Authorization".to_string(),
      value: format!("Bearer {}", token).to_string(),
    },
    HttpHeader {
      name: "Dropbox-API-Arg".to_string(),
      // path: dst from dropbox folder.
      value: format!("{{\"path\":\"/taxlint/payload_{}.json\"}}", timestamp),
    },
  ];

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: None,
    transform: None, //optional for request
  };

  let cycles = 1 * TERA;

  match http_request(request, cycles).await {
    Ok((response,)) => {
      let mut str_body = String::from_utf8(response.body)
        .expect("Transformed response is not UTF-8 encoded.");
      str_body = str_body.replace("\\", "");
      str_body
    }
    Err((r, m)) => {
      let message =
      format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
      message
    }
  }
}

// TODO only ipv6 can do in canister http calls.
// TODO use a ipv6 server to delegate my ipv4 http call.
// make http-ipv4 into http*s*-ipv6
#[ic_cdk::update]
pub async fn get_payload_from_my_server() -> String {
  let host = "www.btwl2333.top";
  let url =
    format!("https://{}/file/file/ret_to_res_body/payload_02.json", host);
  //  https://23.95.213.230:8002/file/file/ret_to_res_body/payload_02.json
  //  https://www.btwl2333.top:8002/file/file/ret_to_res_body/payload_02.json
  let request_headers = vec![HttpHeader {
    name: "Host".to_string(),
    value: format!("{host}:443"),
  }];

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: None,
    transform: None, //optional for request
  };

  let cycles = 1 * TERA;

  match http_request(request, cycles).await {
    Ok((response,)) => {
      let mut str_body = String::from_utf8(response.body)
        .expect("Transformed response is not UTF-8 encoded.");
      str_body = str_body.replace("\\", "");
      str_body
    }
    Err((r, m)) => {
      let message =
      format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
      message
    }
  }
}

#[ic_cdk::update]
pub async fn get_payload_from_my_server_raw_ip() -> String {
  let host = "23.95.213.230";
  let url =
    " https://23.95.213.230:8002/file/file/ret_to_res_body/payload_02.json";

  let request_headers = vec![HttpHeader {
    name: "Host".to_string(),
    value: format!("{host}:8002"),
  }];

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: None,
    transform: None, //optional for request
  };

  let cycles = 1 * TERA;

  match http_request(request, cycles).await {
    Ok((response,)) => {
      let mut str_body = String::from_utf8(response.body)
        .expect("Transformed response is not UTF-8 encoded.");
      str_body = str_body.replace("\\", "");
      str_body
    }
    Err((r, m)) => {
      let message =
      format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
      message
    }
  }
}

#[ic_cdk::update]
pub async fn get_payload_from_dropbox_u8(
  token: String,
  timestamp: String,
) -> Result<Vec<u8>, String> {
  let host = "content.dropboxapi.com";
  let url = "https://content.dropboxapi.com/2/files/download";

  let request_headers = vec![
    HttpHeader {
      name: "Host".to_string(),
      value: format!("{host}:443"),
    },
    HttpHeader {
      name: "Authorization".to_string(),
      value: format!("Bearer {}", token).to_string(),
    },
    HttpHeader {
      name: "Dropbox-API-Arg".to_string(),
      // path: dst from dropbox folder.
      value: format!("{{\"path\":\"/taxlint/payload_{}.json\"}}", timestamp),
    },
  ];

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: None,
    transform: None, //optional for request
  };

  let cycles = 1 * TERA;

  match http_request(request, cycles).await {
    Ok((response,)) => Ok(response.body),
    Err((r, m)) => {
      let message =
      format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
      Err(message)
    }
  }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Utc;

//     #[test]
//     fn test_convert_timestamp() {
//         let now = Utc::now();
//         let timestamp_ns = now.timestamp_nanos() as u64;
//         let converted_time = convert_timestamp(timestamp_ns);
//         assert_eq!(converted_time, now.to_string());
//     }
// }

#[cfg(test)]
mod tests {
  use crate::common::context::{CanisterContext, CanisterDB};

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

  #[test]
  fn gen_new_struct() {
    let db_json = read_db_to_string_from_local_json_file(
      "/home/btwl/code/ic/tax_lint/backend/i_test/pl01_copy.json".to_owned(),
    );
    let payload: CanisterDB =
      serde_json::from_str::<CanisterDB>(&db_json).expect("deseialize old err");
    let ctx = CanisterContext::from(payload);
    let new_struct = serde_json::to_string(&ctx).unwrap();
    std::fs::write("new_struct.json", &new_struct)
      .expect("Unable to write file");
    let payload_result: Result<CanisterContext, _> =
      serde_json::from_str(&new_struct);
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
