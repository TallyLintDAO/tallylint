use ic_cdk_macros::*;

use tracing::info;

use super::context::{CanisterContext, CanisterDB};
use super::env::CanisterEnvironment;
use super::memory::get_upgrades_memory;
use crate::CONTEXT;

use stable_memory::*;

#[allow(dead_code)]
fn init() {
  ic_cdk::setup();
  let context = CanisterContext {
    env: Box::new(CanisterEnvironment {}),
    ..CanisterContext::default()
  };
  let _now = context.env.now();
  // let _creator1 = GOVERNANCE_BTWL.with(|g| *g);
  // let _creator2 = GOVERNANCE_ZHOU.with(|g| *g);

  info!("canister initialization complete");
}
