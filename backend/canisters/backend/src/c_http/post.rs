// 1. IMPORT IC MANAGEMENT CANISTER
//This includes all methods and types needed
use ic_cdk::api::management_canister::http_request::{
  http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

use serde::{Deserialize, Serialize};

use crate::{
  calculate_cost,
  common::life_cycle::{collect_running_payload, get_payload_from_stable_mem},
};

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

// BUG blocking will cause exec too long . fail to exec. dont know fix way yet
#[ic_cdk::update]
pub fn save_payload_to_dropbox_blocking() -> String {
  let host = "content.dropboxapi.com";
  let url = "https://content.dropboxapi.com/2/files/upload";

  let time = ic_cdk::api::time().to_string();

  let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: "Bearer sl.Bv37s812HWeKYX-BMUf9RgSci2voKDRiBRDDoFdpBBKZ7owjmK2Lgm8ZpBm2ehHKim9b1m7qQuDRLZj4RKILREX117jTFxNK9lrwFTBKOXtRMxiA04InrcyTSKqd8Zp2qxc3J2Bq1Buio-cPAzluJwg".to_string(),
        },

        HttpHeader {
        name: "Dropbox-API-Arg".to_string(),

        value: format!("{{\"autorename\":false,\"mode\":\"add\",\"mute\":false,\"path\":\"/taxlint/payload_{}.json\",\"strict_conflict\":false}}", time),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            // value: "application/json".to_string(),
            value: "application/octet-stream".to_string(),
        },
    ];

  let json_string: String = collect_running_payload();

  let json_utf8: Vec<u8> = json_string.into_bytes();
  let json_length = json_utf8.len() as u64;
  let request_body: Option<Vec<u8>> = Some(json_utf8);

  let _context = Context {
    bucket_start_time_index: 0,
    closing_price_index: 4,
  };

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: request_body,
    transform: None, //optional for request
  };

  let cycles = calculate_cost(16, json_length, 1000);

  use futures::executor::block_on;

  match block_on(http_request(request, cycles)) {
    // 4. DECODE AND RETURN THE RESPONSE

    //See:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
    Ok((response,)) => {
      //if successful, `HttpResponse` has this structure:
      // pub struct HttpResponse {
      //     pub status: Nat,
      //     pub headers: Vec<HttpHeader>,
      //     pub body: Vec<u8>,
      // }

      //We need to decode that Vec<u8> that is the body into readable text.
      //To do this, we:
      //  1. Call `String::from_utf8()` on response.body
      //  3. We use a switch to explicitly call out both cases of decoding the
      //     Blob into ?Text
      let str_body = String::from_utf8(response.body)
        .expect("Transformed response is not UTF-8 encoded.");
      ic_cdk::api::print(format!("{:?}", str_body));

      //The API response will looks like this:
      // { successful: true }

      //Return the body as a string and end the method
      let result: String = format!(
        "{}. See more info of the request sent at: {}/inspect",
        str_body, url
      );
      result
    }
    Err((r, m)) => {
      let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

      //Return the error as a string and end the method
      message
    }
  }
}

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

  let json_string: String = collect_running_payload();

  let json_utf8: Vec<u8> = json_string.into_bytes();
  let json_length = json_utf8.len() as u64;

  let request = CanisterHttpRequestArgument {
    url: url.to_string(),
    max_response_bytes: None, //optional for request
    method: HttpMethod::POST,
    headers: request_headers,
    body: None,
    transform: None, //optional for request
  };

  let cycles = calculate_cost(16, json_length, 1000);

  match http_request(request, cycles).await {
    Ok((response,)) => {
      let str_body = String::from_utf8(response.body)
        .expect("Transformed response is not UTF-8 encoded.");
      ic_cdk::api::print(format!("{:?}", str_body));
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
