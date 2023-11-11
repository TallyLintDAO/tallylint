#![cfg_attr(
  debug_assertions,
  allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
use candid::Principal;
use candid::{self, CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::{query, update};
use serde::Serialize;

/*
independent query neuron info .
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
