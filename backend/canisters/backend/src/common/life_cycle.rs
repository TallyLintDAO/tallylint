use ic_cdk::storage;
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use super::context::CanisterContext;
#[allow(unused_imports)]
use candid::{CandidType, Principal};
use crate::STATE;
#[pre_upgrade]
fn pre_upgrade() {
    let state = STATE.with(|s| s.borrow().clone());
    if let state = state {
        if let Err(err) = storage::stable_save((state,)) {
            ic_cdk::println!("Failed to save state during pre_upgrade: {:?}", err);
            // Handle the error, such as logging or taking other actions
        }
    }
}

#[post_upgrade]
fn post_upgrade() {
    if let Ok((state,)) = storage::stable_restore::<(CanisterContext,)>() {
        STATE.with(|s| *s.borrow_mut() = state);
    } else {
        ic_cdk::println!("Failed to restore state during post_upgrade");
        // Handle the error, such as setting a default state or other actions
    }
}



