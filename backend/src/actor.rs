use std::iter::FromIterator;

use ic_cdk::storage;
use ic_cdk::{caller, id, print};
use ic_cdk_macros::*;

use crate::context::{CanisterDB, DaoContext};

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
fn init() {
    println!(" deploy`s init calling");
    ic_cdk::setup();

    let context = DaoContext {
        env: Box::new(CanisterEnvironment {}),
        ..DaoContext::default()
    };

    let _now = context.env.now();
    let _creator1 = GOVERNANACE_BTWL.with(|g| *g);
    let _creator2 = GOVERNANACE_ZHOU.with(|g| *g);

    // with is a function can receive a function as para.
    // and | | synyax here means a function with no name.
    CONTEXT.with(|c| {
        *c.borrow_mut() = context;

        // debug part
        let context = c.borrow();
        let id = context.id;
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        let db: CanisterDB = CanisterDB { id, users };
        println!("{:#?}", db);
    });
    // borrow means temporary need control role of that data.
    // mut means need change the data with in .

    println!("finish  deploy`s init");
}

/**
 * 1. each time upgrade(cmd : dfx deploy ),
 * will *erase* all ic-DB (canister stable memory)
 * so we can:
 *      1.manully erase all,
 *      2.or , restore from a in memory data.(such as a hashmap)
 *
 *  
 * 2. transational upgrade:
 * if pre_upgrade, upgrade ,post_upgrade
 * any step go wrong.
 * will revert to last version.
 *
 *
 */
#[pre_upgrade]
fn pre_upgrade() {
    let canister_id = id();
    print(format!("starting pre_upgrade {:?}", canister_id));

    CONTEXT.with(|c| {
        let context = c.borrow();
        let id = context.id;
        // get users list from vilotile storage (computer memory)
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        let db: CanisterDB = CanisterDB { id, users };
        // println!("{:#?}", db);
        // IMPORTANT save all userdata into IC-DB
        storage::stable_save((db,)).expect("failed to save state data");

        // IMPORTANT erase db in running canister.(ic or local)
        // let _empty_db = CanisterDB::default();
        // storage::stable_save((_empty_db,)).expect("failed to save state data");
        print(format!("started pre_upgrade {:?}", canister_id));
    });
}

#[post_upgrade]
fn post_upgrade() {
    let canister_id = id();
    print(format!("starting post_upgrade {:?}", canister_id));

    // bug here can find restore file.only if no DB data at all. of course cant restore.
    // IMPORTANT
    let (db,): (CanisterDB,) = storage::stable_restore().expect("failed to restore users");
    let state_stable = DaoContext::from(db);
    CONTEXT.with(|s| {
        let mut state = s.borrow_mut();
        *state = state_stable;
    });
    print(format!("finished post_upgrade {:?}", canister_id));
}
