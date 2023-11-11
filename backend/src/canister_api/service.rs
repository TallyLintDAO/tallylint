use candid::{Nat, Principal};
use ic_cdk::api::management_canister::{
  main::{CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument, WasmModule},
  provisional::CanisterSettings,
};

pub fn new_canister_args() -> CreateCanisterArgument {
  let caller = ic_cdk::caller();
  let mut callers = Vec::new();
  callers.push(caller);
  let canister_setting = CanisterSettings {
    controllers: Some(callers),             // Set the caller as the controller
    compute_allocation: Some(Nat::from(0)), // Use default compute allocation
    memory_allocation: Some(Nat::from(0)),  // default
    freezing_threshold: Some(Nat::from(2592000)), // Use default freezing threshold
  };
  let args = CreateCanisterArgument {
    settings: Some(canister_setting),
  };
  return args;
}
pub fn new_install_info(canister_id: Principal) -> InstallCodeArgument {
  let mut wasm: WasmModule = get_wasm_module(); // Create a new empty vector of u8 and assign it to wasm
  wasm = Vec::new(); // create an empty value of type WasmModule
  let mut arg: Vec<u8> = Vec::new(); // Create a new empty vector of u8
  arg.extend_from_slice(b"canister_init"); // Add the bytes of the string "canister_init" to the end of the vector

  let install_config = InstallCodeArgument {
    mode: CanisterInstallMode::Install, // Use install mode
    wasm_module: wasm,                  // Pass in the wasm code
    canister_id: canister_id,           // Pass in the canister id
    arg: arg,                           // Pass in the initialization arguments
  };
  return install_config;
}

//todo: get the all current wasm module. include front end and backend .
// they are .wasm file . how to revoke himself of the running code ?
fn get_wasm_module() -> Vec<u8> {
  // get from on-chain-DB. or local machine file .
  // 1. save the current .wasm file into db.
  // 2. get the wasm from db and treat as a vec<u8> bytes.
  // why the code is running on the ic -chain can read local file ?? wasm ? and ic blockchain application ?
  let bytes: &[u8] = std::include_bytes!("../../../.dfx/local/canisters/backend/backend.wasm");
  let mut wasm = bytes.to_vec();
  // wasm.clear(); // the return is too large !    //why ?
  return wasm;
}
