// 下列代码可实现debug阶段(cargo run或cargo
// build)没有告警，但是生成发布文件(cargo build
// --release)的时候继续告警，
#![cfg_attr(
  debug_assertions,
  allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::{
  main::{
    canister_info, canister_status, create_canister, install_code,
    update_settings, CanisterInfoRequest, CanisterInfoResponse,
    CanisterStatusResponse,
  },
  provisional::{CanisterIdRecord, CanisterSettings},
};
use ic_cdk_macros::{query, update};

use super::service::{new_canister_args, new_install_info};

/**
 * 从罐子里面调用api创建罐子的api.
 * 类比Java的反射. 运行时获取自身所有数据结构并创建
 *
 */
#[update]
pub async fn create_and_install() -> String {
  let cycles = 4_00_000_000_000; //4TC
  let create_res = create_canister(new_canister_args(), cycles).await;
  if create_res.is_err() {
    let _info = create_res
      .map_err(|message| format!("{}", message.1))
      .unwrap_err();
    return String::from("create canister error") + &_info;
  }
  //  CallResult<(CanisterIdRecord,)> : Result----unwrap
  // ()----tuple .0 to get the first tuple.
  #[warn(unused_mut)]
  let mut new_canister_principal = create_res.unwrap().0.canister_id;
  // new_canister_principal=ic_cdk::caller();
  let result = install_code(new_install_info(new_canister_principal)).await;
  if result.is_err() {
    let _info = result
      .map_err(|message| format!(" message {}", message.1))
      .unwrap_err();
    return String::from("install_code error: ") + &_info;
  }
  //   "install_code error:  message Only the controllers of
  // the canister avqkn-guaaa-aaaaa-qaaea-cai can control
  // it.\nCanister\'s controllers:
  // b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae\nSender\'s
  // ID: bkyz2-fmaaa-aaaaa-qaaaq-cai",
  return String::from("create_and_install succuess");
}
#[query]
pub fn whoami() -> Principal {
  return ic_cdk::caller();
}
#[update] // costs cycles. so need update function
pub async fn get_canister_info(id: String) -> CanisterInfoResponse {
  let canister_id = Principal::from_text(id).unwrap();
  let num_requested_changes = Some(10); // Set the desired number of requested changes
  let req = CanisterInfoRequest {
    canister_id,
    num_requested_changes,
  };
  let ret = canister_info(req).await.unwrap().0;
  return ret;
}
#[update]
pub async fn get_canister_status(id: String) -> CanisterStatusResponse {
  let canister_id = Principal::from_text(id).unwrap();

  let rec = CanisterIdRecord { canister_id };
  return canister_status(rec).await.unwrap().0;
}
