use crate::context::DaoContext;
use std::cell::RefCell;
use candid::Principal;
pub mod actor;
pub mod common;
pub mod context;
pub mod env;
mod user;
thread_local! {
  static CONTEXT: RefCell<DaoContext> = RefCell::default();
  static GOVERNANACE_ZHOU : Principal = Principal::from_text("ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae").unwrap();
  static GOVERNANACE_BTWL : Principal = Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae").unwrap();
}
// new auto did generate: in ic_cdk 0.10.0
// IMPORTANT :dont remove this  UserRegisterCommand ref 
use crate::user::domain::UserRegisterCommand;
use crate::user::domain::UserProfile;
use ic_cdk::export_candid;
export_candid!();

