use std::cell::RefCell;

pub mod c2c;
pub mod c_http;
pub mod common;
pub mod lifecycle;
pub mod nns;
pub mod tools;
pub mod transaction;
pub mod user;
pub mod wallet;




#[allow(unused_imports)]
use crate::nns::api::*;
#[allow(unused_imports)]
use crate::nns::domain::*;
#[allow(unused_imports)]
use crate::user::domain::*;
// #[allow(unused_imports)]
// use crate::c2c::oc_wallet::*;
#[allow(unused_imports)]
use crate::c_http::api::*;
#[allow(unused_imports)]
use crate::transaction::domain::SimpleTransaction;
#[allow(unused_imports)]
use crate::transaction::domain::SyncTransactionCommand;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionB;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionF;

#[allow(unused_imports)]
use crate::transaction::domain::MySummary;
#[allow(unused_imports)]
use crate::wallet::domain::*;
#[allow(unused_imports)]
use crate::wallet::service::RecordId;
#[allow(unused_imports)]
use crate::wallet::service::WalletAddress;
#[allow(unused_imports)]
use crate::wallet::service::WalletId;
#[allow(unused_imports)]
use ic_cdk::api::call::CallResult;
#[allow(unused_imports)]
use ic_cdk::api::management_canister::http_request::*;
#[allow(unused_imports)]
use ic_cdk::api::management_canister::{
  main::{
    canister_info, canister_status, create_canister, install_code,
    update_settings, CanisterInfoRequest, CanisterInfoResponse,
    CanisterStatusResponse,
  },
  provisional::{CanisterIdRecord, CanisterSettings},
};
#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::lifecycle::init::CONTEXT;
#[allow(unused_imports)]
use crate::common::context::CanisterContext;



ic_cdk::export_candid!();

// #[cfg(test)]
// mod tests {
//   use super::*;

//   macro_rules! my_add {
//     ($a:expr, $b:expr) => {
//       $a + $b
//     };
//   }

//   #[test]
//   fn test2() {
//     let ret = my_add!(1, 2);
//     eprint!("{}", ret);
//       extern crate proc_macro;
//   use proc_macro::TokenStream;

//   #[proc_macro]
//   pub fn my_macro1(_input: TokenStream) -> TokenStream {
//     "println!(\"Hello, World!\");".parse().unwrap()
//   }
//     my_macro1!();
//   }
// }
