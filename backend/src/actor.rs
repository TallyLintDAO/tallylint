use std::iter::FromIterator;

use ic_cdk::storage;
use ic_cdk::{caller, id, print};
use ic_cdk_macros::*;

use crate::context::{DaoContext, DaoDataStorage};

use crate::env::CanisterEnvironment;
use crate::{CONTEXT, GOVERNANACE_BTWL, GOVERNANACE_ZHOU};

#[query]
fn next_id() -> u64 {
    CONTEXT.with(|s| s.borrow().id)
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn get_caller() -> String {
    caller().to_string()
}

#[query]
fn now() -> u64 {
    CONTEXT.with(|c| c.borrow().env.now())
}

#[init]
fn init_canister() {
    ic_cdk::setup();

    let context = DaoContext {
        env: Box::new(CanisterEnvironment {}),
        ..DaoContext::default()
    };

    let now = context.env.now();
    let creator1 = GOVERNANACE_BTWL.with(|g| *g);
    let creator2 = GOVERNANACE_ZHOU.with(|g| *g);

    CONTEXT.with(|c| {
        *c.borrow_mut() = context;
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let canister_id = id();
    print(format!("starting pre_upgrade {:?}", canister_id));

    CONTEXT.with(|c| {
        let context = c.borrow();
        let id = context.id;
        // get users list from vilotile memory. just computer memory. hash structure
        let users = Vec::from_iter(context.user_service.users.values().cloned());

        let payload: DaoDataStorage = DaoDataStorage { id, users };

        // save all userdata into IC-DB IMPORTANT
        storage::stable_save((payload,)).expect("failed to save state data");

        print(format!("started pre_upgrade {:?}", canister_id));
    });
}

#[post_upgrade]
fn post_upgrade() {
    let canister_id = id();
    print(format!("starting post_upgrade {:?}", canister_id));

// bug here can find restore file.only if no DB data at all. of course cant restore.
// IMPORTANT
    let (payload,): (DaoDataStorage,) = storage::stable_restore().expect("failed to restore users");
    let state_stable = DaoContext::from(payload);

    CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = state_stable;
    });

    print(format!("started post_upgrade {:?}", canister_id));
}

ic_cdk::export::candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
