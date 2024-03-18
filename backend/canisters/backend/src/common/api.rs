use super::context::CanisterContext;
use super::env::CanisterEnvironment;
use ic_cdk::update;
use tracing::info;

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


// #[update(guard = "user_owner_guard")]
// fn get_a_global_id()-> {
//   ic_cdk::setup();
//   let context = CanisterContext {
//     env: Box::new(CanisterEnvironment {}),
//     ..CanisterContext::default()
//   };
//   let _now = context.env.now();
//   // let _creator1 = GOVERNANCE_BTWL.with(|g| *g);
//   // let _creator2 = GOVERNANCE_ZHOU.with(|g| *g);

//   info!("canister initialization complete");
// }
