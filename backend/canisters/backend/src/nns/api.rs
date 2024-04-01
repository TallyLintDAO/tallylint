#![cfg_attr(
  debug_assertions,
  allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
use std::{borrow::BorrowMut, collections::BTreeMap};

use crate::STATE;
use candid::Principal;
#[allow(unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::time;
use ic_cdk::{api::call::CallResult, caller};
use ic_cdk_macros::{query, update};
use serde::Serialize;

/*
independent query neuron info .
TODO: just basic info. need auth or hotkey to get detailed info.
*/
#[update]
pub async fn get_neuron_info(neuron_id: u64) -> CallResult<(CustomResult1,)> {
  return _get_neuron_info(neuron_id).await;
}

pub async fn _get_neuron_info(arg0: u64) -> CallResult<(CustomResult1,)> {
  // maybe get_neuron_info only can call inside ic .not
  // exposed api. call to another canister : on local
  // replica or main net cross networks. maybe cant use
  // local canister call mainnet`s subnet canister ?
  let id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
  ic_cdk::call(id, "get_neuron_info", (arg0,)).await
}
//

#[derive(Serialize, CandidType, Deserialize)]
pub struct NeuronInfo {
  pub dissolve_delay_seconds: u64,
  pub recent_ballots: Vec<BallotInfo>,
  pub created_timestamp_seconds: u64,
  pub state: i32,
  pub stake_e8s: u64,
  pub joined_community_fund_timestamp_seconds: Option<u64>,
  pub retrieved_at_timestamp_seconds: u64,
  pub known_neuron_data: Option<KnownNeuronData>,
  pub voting_power: u64,
  pub age_seconds: u64,
}
#[derive(Serialize, CandidType, Deserialize)]
pub struct KnownNeuronData {
  pub name: String,
  pub description: Option<String>,
}
#[derive(Serialize, CandidType, Deserialize)]
pub struct BallotInfo {
  pub vote: i32,
  pub proposal_id: Option<NeuronId>,
}
#[derive(Serialize, Clone, Debug, CandidType, Deserialize)]
pub struct NeuronId {
  pub id: serde_bytes::ByteBuf,
}
#[derive(Serialize, CandidType, Deserialize)]
pub enum CustomResult1 {
  Ok(NeuronInfo),
  Err(GovernanceError),
}
#[derive(Serialize, Clone, Debug, CandidType, Deserialize)]
pub struct GovernanceError {
  pub error_message: String,
  pub error_type: i32,
}

use crate::{
  common::guard::user_owner_guard, nns::domain::*,
  wallet::domain::WalletProfile,
};

use super::service::NeuronService;

#[update(guard = "user_owner_guard")]
fn add_neuron_wallet(cmd: NeuronAddCommand) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let user = caller();
    let time = time();
    let id = ctx.id;

    let mut service = ctx.neuron_service.borrow_mut();
    let addr = cmd.address.clone();
    if service.neurons.contains_key(&addr) {
      return Err("neuron addr duplicated".to_string());
    }

    let profile = NeuronProfile {
      owner: user,
      name: cmd.name,
      id: id,
      create_time: time,
      address: cmd.address,
      update_time: time,
    };
    let ret = service.neurons.insert(addr, profile);
    ctx.id = id + 1;
    return Ok(true);
  })
}

#[update(guard = "user_owner_guard")]
fn delete_neuron_wallet(id: u64) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let user = caller();
    let mut service = ctx.neuron_service.borrow_mut();
    let profile = service.search_by_id(id);
    if profile.is_none() {
      return Err("no neuron find by id ".to_string());
    }
    service.neurons.remove(&profile.unwrap().address.clone());
    return Ok(true);
  })
}

#[update(guard = "user_owner_guard")]
fn update_neuron_wallet(cmd: NeuronUpdateCommand) -> Result<bool, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let user = caller();
    let mut service = ctx.neuron_service.borrow_mut();
    let mut profile = service.search_by_id(cmd.id);
    if profile.is_none() {
      return Err("no neuron find by id ".to_string());
    }
    let mut p: NeuronProfile = profile.unwrap().clone();
    p.name = cmd.name;
    service.neurons.insert(p.address.clone(), p.to_owned());
    return Ok(true);
  })
}

#[query(guard = "user_owner_guard")]
fn query_all_neuron_wallet() -> Result<Vec<NeuronProfile>, Vec<NeuronProfile>> {
  STATE.with(|c| {
    let ctx = c.borrow_mut();
    let user = caller();
    let neurons = ctx
      .neuron_service
      .search_by_owner(user)
      .into_iter()
      .cloned()
      .collect();
    return Ok(neurons);
  })
}

#[query(guard = "user_owner_guard")]
fn query_a_neuron_wallet(id: u64) -> Result<NeuronProfile, String> {
  STATE.with(|c| {
    let mut ctx = c.borrow_mut();
    let user = caller();
    let mut ns = ctx.neuron_service.borrow_mut();
    let a_neuron = ns.search_by_id(id);
    if a_neuron.is_none() {
      return Err("no neuron find by id".to_string());
    }
    return Ok(a_neuron.unwrap().clone());
  })
}
