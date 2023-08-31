#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
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

// ic-onchain-canister-counter example code :
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

// gpt api example
// gpt api .not compatible with ic-sdk yet. compile error
// use chatgpt::prelude::*;

// #[ic_cdk::query]
// async fn gpt(prompt: String) -> Result<String> {
//     let client = ChatGPT::new(String::from(
//         "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",
//     ))?;
//     let response = client.send_message(&prompt).await?;
//     return Ok(response.message().content.to_string());
// }
