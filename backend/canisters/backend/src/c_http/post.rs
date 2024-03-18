// 1. IMPORT IC MANAGEMENT CANISTER
//This includes all methods and types needed
use ic_cdk::api::management_canister::http_request::{
  http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

use serde::{Deserialize, Serialize};

use crate::{calculate_cost, common::life_cycle::collect_running_payload};
pub const TERA: Cycles = 1_000_000_000_000;
pub type Cycles = u128;

// This struct is legacy code and is not really used in the code.
#[derive(Serialize, Deserialize)]
struct Context {
  bucket_start_time_index: usize,
  closing_price_index: usize,
}
// use chrono::{DateTime, Utc};

// pub fn convert_timestamp(timestamp_ns: u64) -> String {
//     let timestamp_s = timestamp_ns / 1_000_000_000;
//     let naive_datetime =
// chrono::NaiveDateTime::from_timestamp_opt(timestamp_s as i64, 0).unwrap();
//     let datetime: DateTime<Utc> =
// DateTime::from_naive_utc_and_offset(naive_datetime, Utc);     datetime.
// to_string() }

//Update method using the HTTPS outcalls feature

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
