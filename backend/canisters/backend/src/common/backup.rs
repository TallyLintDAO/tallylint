use ic_cdk::{query,update};
use candid::{CandidType, Deserialize, Principal};
use crate::common::guard::admin_guard;
use super::context::CanisterContext;
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static STATE: RefCell<BTreeMap<PrincipalStorable, CanisterContext>> = RefCell::new(BTreeMap::new());
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, CandidType, Deserialize)]
struct PrincipalStorable(Principal);


#[query(guard = "admin_guard")]
#[candid::candid_method]
fn backup(offset: u32, count: u32) -> Vec<(String, CanisterContext)> {
    STATE.with(|p| {
        p.borrow()
            .iter()
            .skip(offset as usize)
            .take(count as usize)
            .map(|(k, p)| (k.0.to_text(), p.clone()))
            .collect()
    })
}

#[update(guard = "admin_guard")]
#[candid::candid_method]
fn restore(profiles: Vec<(String, CanisterContext)>) {
    STATE.with(|m| {
        let mut m = m.borrow_mut();
        for p in profiles {
            let principal = PrincipalStorable(Principal::from_text(p.0).unwrap());
            m.insert(principal, p.1).unwrap();
        }
    });
}





