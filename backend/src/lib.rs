/*
basic hello world
 */
#[ic_cdk::query]
fn greet(name: String) -> String {
    // format!("Hello, {}!", name)
    format!("Hi there, {}!", name)
}

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static MAP: RefCell<StableBTreeMap<u128, u128, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

// Retrieves the value associated with the given key if it exists.
// #[ic_cdk::query] // #[ic_cdk_macros::query] // whats the diff ??
#[ic_cdk_macros::query]
fn get(key: u128) -> Option<u128> {
    MAP.with(|p| p.borrow().get(&key))
}

// Inserts an entry into the map and returns the previous value of the key if it exists.
#[ic_cdk_macros::update]
fn insert(key: u128, value: u128) -> Option<u128> {
    MAP.with(|p| p.borrow_mut().insert(key, value))
}

/*
ic-onchain-canister-counter example code :
*/
use candid::types::number::Nat;
// use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

/// Get the value of the counter.
#[ic_cdk_macros::query]
fn get_time() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Set the value of the counter.
#[ic_cdk_macros::update]
fn set(n: Nat) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    COUNTER.with(|count| *count.borrow_mut() = n);
}

/// Increment the value of the counter.
#[ic_cdk_macros::update]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_time_set() {
        let expected = Nat::from(42);
        set(expected.clone());
        assert_eq!(get_time(), expected);
    }

    #[test]
    fn test_init() {
        assert_eq!(get_time(), Nat::from(0));
    }

    #[test]
    fn test_inc() {
        for i in 1..10 {
            inc();
            assert_eq!(get_time(), Nat::from(i));
        }
    }
}

/*
update rust canister example
*/
use ic_cdk::{
    api::call::ManualReply, export::Principal, init, post_upgrade, pre_upgrade, query, storage,
    update,
};
// use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

type Users = BTreeSet<Principal>;
type Store = BTreeMap<String, Vec<u8>>;

thread_local! {
    static USERS: RefCell<Users> = RefCell::default();
    static STORE: RefCell<Store> = RefCell::default();
}

#[init]
fn init() {
    USERS.with(|users| users.borrow_mut().insert(ic_cdk::api::caller()));
}

fn is_user() -> Result<(), String> {
    if USERS.with(|users| users.borrow().contains(&ic_cdk::api::caller())) {
        Ok(())
    } else {
        Err("Store can only be set by the owner of the asset canister.".to_string())
    }
}

#[update(guard = "is_user")] // beautiful guard key word semantics !
fn store(path: String, contents: Vec<u8>) {
    STORE.with(|store| store.borrow_mut().insert(path, contents));
}

#[query(manual_reply = true)]
fn retrieve(path: String) -> ManualReply<Vec<u8>> {
    STORE.with(|store| match store.borrow().get(&path) {
        Some(content) => ManualReply::one(content),
        None => panic!("Path {} not found.", path),
    })
}

#[update(guard = "is_user")]
fn add_user(principal: Principal) {
    USERS.with(|users| users.borrow_mut().insert(principal));
}

#[pre_upgrade]
fn pre_upgrade() {
    USERS.with(|users| storage::stable_save((users,)).unwrap());
}

// #[post_upgrade]
// fn post_upgrade() {
//     let (old_users,): (BTreeSet<Principal>,) = storage::stable_restore().unwrap();
//     USERS.with(|users| *users.borrow_mut() = old_users);
// }

