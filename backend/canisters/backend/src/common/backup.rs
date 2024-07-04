use crate::{common::guard::admin_guard};
use ic_cdk_macros::{query,update};


#[query(guard = "admin_guard")]
#[candid::candid_method]
fn backup(offset: u32, count: u32) -> String {
    String::new()
}

#[query(guard = "admin_guard")]
#[candid::candid_method]
fn restore(offset: u32, count: u32) -> String {
    String::new()
}




