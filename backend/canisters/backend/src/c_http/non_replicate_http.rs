use std::{borrow::Borrow, cell::RefCell, collections::HashMap};

use candid::Principal;
use ic_cdk::{caller, trap};
use ic_cdk_macros::{query, update};
use proxy_canister_types::{
  HttpHeader, HttpMethod, HttpRequest, HttpRequestEndpointArgs,
  HttpRequestEndpointResult, HttpRequestId, HttpRequestTimeoutMs, HttpResult,
};

use crate::common::life_cycle::get_payload;

thread_local! {
    /* flexible */ static PROXY_CANISTER_ID: RefCell<Principal> = RefCell::new(Principal::anonymous());
    /* flexible */ static CALLBACK_RESPONSES: RefCell<HashMap<HttpRequestId, HttpResult>> = RefCell::new(HashMap::new());
}

pub fn http_init(proxy_canister_id: Principal) {
  PROXY_CANISTER_ID.with(|id| {
    id.replace(proxy_canister_id);
  });
}

pub fn http_post_upgrade(proxy_canister_id: Principal) {
  http_init(proxy_canister_id);
}

#[update]
async fn http_request_via_proxy(
  req: HttpRequest,
  timeout_ms: Option<HttpRequestTimeoutMs>,
  with_callback: bool,
) -> HttpRequestEndpointResult {
  let proxy_canister_id = PROXY_CANISTER_ID.with(|id| id.borrow().clone());
  let res: Result<(HttpRequestEndpointResult,), _> = ic_cdk::call(
    proxy_canister_id,
    "http_request",
    (HttpRequestEndpointArgs {
      request: req,
      timeout_ms,
      callback_method_name: with_callback
        .then_some("http_response_callback".to_string()),
    },),
  )
  .await;

  match res {
    Ok(http_res) => http_res.0,
    Err(e) => {
      trap(format!("{:?}", e).as_str());
    }
  }
}

#[update]
fn http_response_callback(request_id: HttpRequestId, res: HttpResult) {
  if caller() != PROXY_CANISTER_ID.with(|id| id.borrow().clone()) {
    trap("Caller is not the proxy canister");
  }

  CALLBACK_RESPONSES.with(|callbacks| {
    let mut callbacks = callbacks.borrow_mut();
    callbacks.insert(request_id, res);
  });
}

#[query]
fn get_http_results() -> HashMap<HttpRequestId, HttpResult> {
  CALLBACK_RESPONSES.with(|responses| responses.borrow().clone())
}

#[query]
fn get_http_result_by_id(request_id: HttpRequestId) -> Option<HttpResult> {
  CALLBACK_RESPONSES
    .with(|responses| responses.borrow().get(&request_id).cloned())
}

#[update]
pub async fn store_paylaod_to_dropbox() -> String {
  //construct request
  let url = String::from("https://content.dropboxapi.com/2/files/upload");
  let method = HttpMethod::POST;
  let headers = vec![
    HttpHeader {
        name: String::from("Authorization"),
        value: String::from("sl.BuJkQn6_xnatK6KB9t5YfJXNQxH4dMm6mAMi2uutjpuKLulYXrstBm7k5qcggSc-Wc9_DTw2t7-csBPj5bXffLxtEEOGdUSlvhzgq3HbZcgzLO4eVmvh-IKEWhZyRrVC-2yskjqakhg_"),
    },
    HttpHeader {
        name: String::from("Dropbox-API-Arg"),
        // path semantics : dst path: Path in the user's Dropbox to save the file.
        value: String::from(r#"{"autorename":false,"mode":"add","mute":false,"path":"/taxlint/payload_bin","stri)ct_conflict":false}"#),
    },
    HttpHeader {
        name: String::from("Content-Type"),
        value: String::from("application/octet-stream"),
    },
];
  let data = get_payload().into_bytes();
  let body = Some(data);
  let request = HttpRequest {
    url,
    method,
    headers,
    body,
  };

  //send http call
  // way1. this must using main-net ic chain.
  let ret = http_request_via_proxy(request.clone(), None, true).await;
  if ret.is_err() {
    return String::from("http request error");
  }
  let request_id = ret.unwrap();
  //get ret:
  get_http_result_by_id(request_id);

  // //  send http call  : way2
  // let cycles = 230_949_972_000; //0.2T
  // let ret2=http_request(expand_to_canister_http_request(request,None,None),
  // cycles).await; if ret2.is_err() {
  //   return String::from("http request error");
  // }
  // let response= ret2.unwrap().0;
  // return response.body.to_ascii_lowercase();

  return String::from("ok");
}

// pub fn expand_to_canister_http_request(req: HttpRequest, max_response_bytes:
// Option<u64>, transform: Option<TransformContext>) ->
// CanisterHttpRequestArgument {     CanisterHttpRequestArgument {
//         url: req.url,
//         max_response_bytes: max_response_bytes,
//         method: req.method,
//         headers: req.headers,
//         body: req.body,
//         transform: transform,
//     }
// }
