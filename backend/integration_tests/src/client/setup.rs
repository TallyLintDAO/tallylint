use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

use candid::{CandidType, Principal};
use pocket_ic::{PocketIc, PocketIcBuilder};

use super::{
  rng::random_principal,
  utils::principal_to_username,
  wasms::{self, CanisterWasm},
};

pub struct TestEnv {
  pub env: PocketIc,
  pub canister_ids: CanisterIds,
  pub controller: Principal,
}
// https://github.com/dfinity/pocketic
//  gunzip xxx.
// sudo ln -s ~/app/pocket-ic /usr/bin/pocket-ic
// TODO pocket-ic dont tell me he got new version , how ?
pub static POCKET_IC_BIN: &str = "/usr/bin/pocket-ic";

pub fn setup_new_env() -> TestEnv {
  let path = match env::var_os("POCKET_IC_BIN") {
    None => {
      env::set_var("POCKET_IC_BIN", POCKET_IC_BIN);
      POCKET_IC_BIN.to_string()
    }
    Some(path) => path
      .clone()
      .into_string()
      .unwrap_or_else(|_| panic!("Invalid string path for {path:?}")),
  };

  if !Path::new(&path).exists() {
    println!("
        Could not find the PocketIC binary to run canister integration tests.

        I looked for it at {:?}. You can specify another path with the environment variable POCKET_IC_BIN (note that I run from {:?}).

        Running the testing script will automatically place the PocketIC binary at the right place to be run without setting the POCKET_IC_BIN environment variable:
            ./scripts/run-integration-tests.sh
        ", &path, &env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_| "an unknown directory".to_string()));
  }

  let mut env = PocketIcBuilder::new()
    .with_nns_subnet()
    .with_application_subnet()
    .build();
  let controller = random_principal();
  let canister_ids = install_canisters(&mut env, controller);

  TestEnv {
    env,
    canister_ids,
    controller,
  }
}
// code behavoir: install canisters(wasm file) into test-env(ic-replica of impl
// by pocketIC) TODO: under constructing
fn install_canisters(env: &mut PocketIc, controller: Principal) -> CanisterIds {
  let backend_canister_wasm = wasms::BACKEND.clone();

  let local_backend_canister_id = create_canister(env, controller);

  #[derive(CandidType, Serialize, Deserialize, Debug)]
  pub struct Args {
    // Only these principals can call upgrade_user_canister_wasm
    pub service_principals: Vec<Principal>,

    pub backend_canister_wasm: CanisterWasm,
    // pub local_user_index_canister_wasm: CanisterWasm,
    // pub group_index_canister_id: CanisterId,
    // pub notifications_index_canister_id: CanisterId,
    // pub proposals_bot_canister_id: CanisterId,
    // pub cycles_dispenser_canister_id: CanisterId,
    // pub storage_index_canister_id: CanisterId,
    // pub internet_identity_canister_id: CanisterId,
    // pub wasm_version: BuildVersion,
    pub test_mode: bool,
  }

  // TODO should be  user_index_canister::init::Args {  in canisters/ folder
  // code . 捡懒
  let backend_init_args = Args {
    service_principals: vec![controller],
    backend_canister_wasm: CanisterWasm::default(),
    // wasm_version: BuildVersion::min(),
    test_mode: true,
  };

  install_canister(
    env,
    controller,
    local_backend_canister_id,
    backend_canister_wasm,
    backend_init_args,
  );
  return CanisterIds {
    backend: local_backend_canister_id,
  };
}

pub fn install_canister<P: CandidType>(
  env: &mut PocketIc,
  sender: Principal,
  canister_id: CanisterId,
  wasm: CanisterWasm,
  payload: P,
) {
  env.install_canister(
    canister_id,
    wasm.module,
    candid::encode_one(&payload).unwrap(),
    Some(sender),
  )
}

const INIT_CYCLES_BALANCE: u128 = 1_000 * TERA;

pub fn create_canister(
  env: &mut PocketIc,
  controller: Principal,
) -> CanisterId {
  let canister_id = env.create_canister_with_settings(Some(controller), None);
  env.add_cycles(canister_id, INIT_CYCLES_BALANCE);
  canister_id
}

#[derive(
  CandidType,
  Serialize,
  Deserialize,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
)]
pub struct UserId(CanisterId);

impl UserId {
  pub const fn new(canister_id: CanisterId) -> UserId {
    UserId(canister_id)
  }
}

impl From<Principal> for UserId {
  fn from(principal: Principal) -> Self {
    UserId(principal)
  }
}

impl From<UserId> for CanisterId {
  fn from(user_id: UserId) -> Self {
    user_id.0
  }
}

impl Debug for UserId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.0, f)
  }
}

impl Display for UserId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.0, f)
  }
}

#[derive(Debug)]
pub struct User {
  pub principal: Principal,
  pub user_id: UserId,
}

impl User {
  pub fn canister(&self) -> CanisterId {
    self.user_id.into()
  }

  pub fn username(&self) -> String {
    principal_to_username(self.principal)
  }
}

pub type CanisterId = Principal;

#[derive(Debug)]
pub struct CanisterIds {
  pub backend: CanisterId,
  // pub group_index: CanisterId,
  // and other new canisters ...
}
pub type Cycles = u128;

// T is not so good. simialr with gererics <T>
pub const TERA: Cycles = 1_000_000_000_000;
pub const NNS_INTERNET_IDENTITY_CANISTER_ID: CanisterId =
  Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 10, 1, 1]);
