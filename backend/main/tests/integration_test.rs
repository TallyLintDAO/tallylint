//! tests is a special folder in rust .

use candid::{CandidType, Decode, Encode, Nat};
use ic_agent::{export::Principal, Agent};
use ic_stable_structures::BTreeMap;
use serde::Deserialize;

#[derive(CandidType)]
struct Argument {
  amount: Option<Nat>,
}

#[derive(CandidType, Deserialize)]
struct CreateCanisterResult {
  canister_id: Principal,
}

async fn create_a_canister() -> Result<Principal, Box<dyn std::error::Error>> {
  // should do this or check if ic-replica is running : dfx
  // start --background and which port is it ? or how to
  // specify ?   Agent:A low level Agent to make calls to
  // a Replica endpoint.
  let agent = Agent::builder()
    .with_url("http://localhost:8000") // Replace with the appropriate URL
    // ! pgrep replica  # get pid from name
    // !  lsof -i -P -n | grep LISTEN | grep <pid>
    // .with_identity(create_identity())
    .build()?;
  // Only do the following call when not contacting the IC
  // main net (e.g. a local emulator). This is important
  // as the main net public key is static and a rogue
  // network could return a different key. If you know the
  // root key ahead of time, you can use
  // `agent.set_root_key(root_key);`.
  agent.fetch_root_key().await?;
  let management_canister_id = Principal::from_text("aaaaa-aa")?;

  // Create a call to the management canister to create a
  // new canister ID, and wait for a result.
  // The effective canister id must belong to the canister
  // ranges of the subnet at which the canister is
  // created.
  let effective_canister_id =
    Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
  let response = agent
    .update(
      &management_canister_id,
      "provisional_create_canister_with_cycles",
    )
    .with_effective_canister_id(effective_canister_id)
    .with_arg(Encode!(&Argument { amount: None })?)
    .call_and_wait()
    .await?;

  let result = Decode!(response.as_slice(), CreateCanisterResult)?;
  let canister_id: Principal = result.canister_id;
  Ok(canister_id)
}

#[tokio::test]
// not #[cfg(test)] //unit-test attribute(#) in rust syntax
// #[tokio::test1]
// async fn test1() {
async fn test1() {
  test_run().await;
}
// if not await , test_run return nothing.

async fn test_run() {
  let canister_id = create_a_canister().await.unwrap();
  eprintln!("{}", canister_id);
}



#[test]
fn test_stable_mem(){
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

const UPGRADES: MemoryId = MemoryId::new(0);
const INSTRUCTION_COUNTS_INDEX: MemoryId = MemoryId::new(1);
const INSTRUCTION_COUNTS_DATA: MemoryId = MemoryId::new(2);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl>
        = MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 16);
}

pub fn get_upgrades_memory() -> Memory {
    get_memory(UPGRADES)
}

pub fn get_instruction_counts_index_memory() -> Memory {
    get_memory(INSTRUCTION_COUNTS_INDEX)
}

pub fn get_instruction_counts_data_memory() -> Memory {
    get_memory(INSTRUCTION_COUNTS_DATA)
}

fn get_memory(id: MemoryId) -> Memory {
    MEMORY_MANAGER.with(|m| m.get(id))
}



let mut map: BTreeMap<u64, u64, _> = BTreeMap::init(DefaultMemoryImpl::default());

map.insert(1, 2);
assert_eq!(map.get(&1), Some(2));

use std::cell::RefCell;
use ic_stable_structures::{StableBTreeMap};

thread_local! {
  static USERS: RefCell<StableBTreeMap<(u32,u32), u32, DefaultMemoryImpl>> =
    RefCell::new(StableBTreeMap::init(DefaultMemoryImpl::default()));
}
USERS.with(|u|{
  let mut users= u.borrow_mut();
  users.insert((10,1), 5);
  
  let ret=users.get(&(10,1));
  assert_eq!(ret,Some(5));
});

let m=get_memory(INSTRUCTION_COUNTS_DATA);




}
