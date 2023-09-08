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
  /// 初始化创始人 principal
  static GOVERNANACE_ZHOU : Principal = Principal::from_text("ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae").unwrap();
  static GOVERNANACE_BTWL : Principal = Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae").unwrap();

  // /// 初始化创始人声望值 1 亿
  // static GOVERNANACE_CREATOR_REPUTATION : u64 = 100_000_000;

}

// ic_cdk::export::candid::export_service!();


// use ic_cdk_macros::*;
// #[query(name = "__get_candid_interface_tmp_hack")]
// fn export_candid() -> String {
//     __export_service()
// }


// #[cfg(test)]
// mod tests {
    
//     #[test]
//     fn did() {

//       use candid::export_service;
      
//       ic_cdk::export::candid::export_service!();
//       #[name = "__get_candid_interface_tmp_hack"]
//       fn export_candid() -> String {
//           export_service!();
//           __export_service()
//       }
      
//         use std::env;
//         use std::fs::write;
//         use std::path::PathBuf;
    
//         let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
//         println!("dir: {:?}", dir);
//         // let dir = dir.parent().unwrap().parent().unwrap().join("candid");
//         // println!("dir: {:?}", dir);
//         write(dir.join("backend.did"), export_candid()).expect("Write failed.");
//         println!("write finish");
//     }
// }



// async fn transfer(cmd: TransferCommand) -> Result<BlockIndex, String> {
//     ic_cdk::println!(
//         "Transferring {} tokens to principal {} ",
//         &cmd.amount_e8s,
//         &cmd.recipient_principal
//     );
//     let ledger_canister_id = MAINNET_LEDGER_CANISTER_ID;
//     let to_subaccount = DEFAULT_SUBACCOUNT;
//     let to_principal =
//         Principal::from_text(cmd.recipient_principal).or(Err("Recipient format is wrong!"))?;

//     let transfer_args = ic_ledger_types::TransferArgs {
//         memo: Memo(0),
//         amount: Tokens::from_e8s(cmd.amount_e8s),
//         fee: DEFAULT_FEE,
//         from_subaccount: None,
//         to: AccountIdentifier::new(&to_principal, &to_subaccount),
//         created_at_time: None,
//     };

//     ic_ledger_types::transfer(ledger_canister_id, transfer_args)
//         .await
//         .map_err(|e| format!("failed to call ledger: {:?}", e))?
//         .map_err(|e| format!("ledger transfer error {:?}", e))
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct TransferCommand {
//     pub amount_e8s: u64,
//     pub recipient_principal: String,
// }

// use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
// use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
// use std::cell::RefCell;

// type Memory = VirtualMemory<DefaultMemoryImpl>;

// thread_local! {
//     // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
//     // return a memory that can be used by stable structures.
//     static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
//         RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

//     // Initialize a `StableBTreeMap` with `MemoryId(0)`.
//     static MAP: RefCell<StableBTreeMap<u128, u128, Memory>> = RefCell::new(
//         StableBTreeMap::init(
//             MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
//         )
//     );
// }

// use ic_cdk_macros::query;
// use ic_cdk_macros::update;
// // Retrieves the value associated with the given key if it exists.
// // #[ic_cdk::query] // #[ic_cdk_macros::query] // whats the diff ??
// #[ic_cdk_macros::query]
// fn get(key: u128) -> Option<u128> {
//     MAP.with(|p| p.borrow().get(&key))
// }

// // Inserts an entry into the map and returns the previous value of the key if it exists.
// #[ic_cdk_macros::update]
// fn insert(key: u128, value: u128) -> Option<u128> {
//     MAP.with(|p| p.borrow_mut().insert(key, value))
// }

// /*
// ic-onchain-canister-counter example code :
// */
// use candid::types::number::Nat;
// // use std::cell::RefCell;

// thread_local! {
//     static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
// }

// /// Get the value of the counter.
// #[ic_cdk_macros::query]
// fn get_time() -> Nat {
//     COUNTER.with(|counter| (*counter.borrow()).clone())
// }

// /// Set the value of the counter.
// #[ic_cdk_macros::update]
// fn set(n: Nat) {
//     // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
//     COUNTER.with(|count| *count.borrow_mut() = n);
// }

// /// Increment the value of the counter.
// #[ic_cdk_macros::update]
// fn inc() {
//     COUNTER.with(|counter| *counter.borrow_mut() += 1);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_time_set() {
//         let expected = Nat::from(42);
//         set(expected.clone());
//         assert_eq!(get_time(), expected);
//     }

//     #[test]
//     fn test_init() {
//         assert_eq!(get_time(), Nat::from(0));
//     }

//     #[test]
//     fn test_inc() {
//         for i in 1..10 {
//             inc();
//             assert_eq!(get_time(), Nat::from(i));
//         }
//     }
// }

// /*
// update rust canister example
// */
// use ic_cdk::{api::call::ManualReply, export::Principal, init, post_upgrade, pre_upgrade, storage};
// // use std::cell::RefCell;
// use std::collections::{BTreeMap, BTreeSet};

// type Users = BTreeSet<Principal>;
// type Store = BTreeMap<String, Vec<u8>>;

// thread_local! {
//     static USERS: RefCell<Users> = RefCell::default();
//     static STORE: RefCell<Store> = RefCell::default();
// }

// #[init]
// fn init() {
//     USERS.with(|users| users.borrow_mut().insert(ic_cdk::api::caller()));
// }

// fn is_user() -> Result<(), String> {
//     if USERS.with(|users| users.borrow().contains(&ic_cdk::api::caller())) {
//         Ok(())
//     } else {
//         Err("Store can only be set by the owner of the asset canister.".to_string())
//     }
// }

// #[update(guard = "is_user")] // beautiful guard key word semantics !
// fn store(path: String, contents: Vec<u8>) {
//     STORE.with(|store| store.borrow_mut().insert(path, contents));
// }

// #[query(manual_reply = true)]
// fn retrieve(path: String) -> ManualReply<Vec<u8>> {
//     STORE.with(|store| match store.borrow().get(&path) {
//         Some(content) => ManualReply::one(content),
//         None => panic!("Path {} not found.", path),
//     })
// }

// #[update(guard = "is_user")]
// fn add_user(principal: Principal) {
//     USERS.with(|users| users.borrow_mut().insert(principal));
// }

// #[pre_upgrade]
// fn pre_upgrade() {
//     USERS.with(|users| storage::stable_save((users,)).unwrap());
// }

// // #[post_upgrade]
// // fn post_upgrade() {
// //     let (old_users,): (BTreeSet<Principal>,) = storage::stable_restore().unwrap();
// //     USERS.with(|users| *users.borrow_mut() = old_users);
// // }
