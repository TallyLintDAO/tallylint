use ic_cdk::storage;
use ic_cdk_macros::*;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use crate::{CONTEXT, GOVERNANACE_BTWL, GOVERNANACE_ZHOU};

#[init]
fn init() {
    ic_cdk::setup();
    let context = CanisterContext {
        env: Box::new(CanisterEnvironment {}),
        ..CanisterContext::default()
    };
    let now = context.env.now();
    let creator1 = GOVERNANACE_BTWL.with(|g| *g);
    let creator2 = GOVERNANACE_ZHOU.with(|g| *g);
}

/**
 * 1. each time upgrade(cmd : dfx deploy ),
 * will *erase* all ic-DB (canister stable memory)
 * so we can:
 *      1.manully erase all,
 *      2.or , restore from a in memory data.(such as a hashmap)
 * 2. transational upgrade:
 * if pre_upgrade, upgrade ,post_upgrade
 * any step go wrong.
 * will revert to last version.
 */
#[pre_upgrade]
fn pre_upgrade() {
    // with is a function can receive a function as para.
    // and | | synyax here means a function with no name.
    CONTEXT.with(|c| {
        let context = c.borrow();
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        let wallets = Vec::from_iter(context.wallet_service.wallets.values().cloned());
        let payload: CanisterDB = CanisterDB { id, users, wallets };
        storage::stable_save((payload,)).expect("failed to save state data");
        // IMPORTANT erase db in running canister.(ic or local)
        // dfx deploy backend --network ic  -m reinstall
    });
}

#[post_upgrade]
fn post_upgrade() {
    // IMPORTANT
    // () means retrive multiple db.
    let (payload,): (CanisterDB,) = storage::stable_restore().expect("failed to restore users");
    let stable_state = CanisterContext::from(payload);
    CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = stable_state;
    });
}
