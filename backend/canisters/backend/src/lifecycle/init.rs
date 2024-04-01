use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// 捡懒.写这里了
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CanisterWasm {
  // pub version: BuildVersion,
  #[serde(with = "serde_bytes")]
  pub module: Vec<u8>,
}
pub type CanisterId = Principal;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
  // Only these principals can call upgrade_user_canister_wasm
  pub service_principals: Vec<Principal>,

  pub user_canister_wasm: CanisterWasm,
  // pub local_user_index_canister_wasm: CanisterWasm,
  // pub group_index_canister_id: CanisterId,
  // pub notifications_index_canister_id: CanisterId,
  // pub proposals_bot_canister_id: CanisterId,
  // pub cycles_dispenser_canister_id: CanisterId,
  // pub storage_index_canister_id: CanisterId,
  // pub internet_identity_canister_id: CanisterId,
  // pub wasm_version: BuildVersion,
  pub test_mode: bool,
}
use crate::common::context::CanisterContext;
use crate::RefCell;
thread_local! {
  pub static CONTEXT: RefCell<CanisterContext> = RefCell::new(CanisterContext::new());
}
