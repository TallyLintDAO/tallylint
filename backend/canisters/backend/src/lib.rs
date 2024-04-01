use crate::common::context::CanisterContext;
use std::cell::RefCell;
thread_local! {
  pub static STATE: RefCell<CanisterContext> = RefCell::new(CanisterContext::new());
}

pub mod c2c;
pub mod c_http;
pub mod common;
pub mod nns;
pub mod tools;
pub mod transaction;
pub mod user;
pub mod wallet;

#[allow(unused_imports)]
use crate::c_http::api::*;
#[allow(unused_imports)]
use crate::nns::api::*;
#[allow(unused_imports)]
use crate::nns::domain::*;
#[allow(unused_imports)]
use crate::transaction::domain::MySummary;
#[allow(unused_imports)]
use crate::transaction::domain::SimpleTransaction;
#[allow(unused_imports)]
use crate::transaction::domain::SyncTransactionCommand;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionB;
#[allow(unused_imports)]
use crate::transaction::domain::TransactionF;
#[allow(unused_imports)]
use crate::user::domain::*;
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

ic_cdk::export_candid!();
