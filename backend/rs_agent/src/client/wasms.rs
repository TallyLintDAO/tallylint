use super::utils::local_bin;
use candid::CandidType;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct CanisterWasm {
  // pub version: BuildVersion,
  #[serde(with = "serde_bytes")]
  pub module: Vec<u8>,
}
impl Default for CanisterWasm {
  fn default() -> Self {
    CanisterWasm {
      // version: BuildVersion::new(0, 0, 0),
      module: Vec::default(),
    }
  }
}
impl Debug for CanisterWasm {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CanisterWasm")
      // .field("version", &self.version)
      .field("byte_length", &self.module.len())
      .finish()
  }
}

// export file to ohter code:
lazy_static! {
  pub static ref BACKEND: CanisterWasm = get_canister_wasm("backend");
}

fn get_canister_wasm(canister_name: &str) -> CanisterWasm {
  let module = read_file_from_local_bin(&format!("{canister_name}.wasm"));

  CanisterWasm {
    // version: BuildVersion::min(),
    module,
  }
}

fn read_file_from_local_bin(file_name: &str) -> Vec<u8> {
  let mut file_path = local_bin();
  file_path.push(file_name);

  let mut file = File::open(&file_path).unwrap_or_else(|_| {
    panic!("Failed to open file: {}", file_path.to_str().unwrap())
  });
  let mut bytes = Vec::new();
  file.read_to_end(&mut bytes).expect("Failed to read file");
  bytes
}
