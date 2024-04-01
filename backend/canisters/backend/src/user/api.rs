use crate::lifecycle::init::CONTEXT;
use crate::{common::guard::admin_guard, UserConfig};
use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::query;
use ic_cdk_macros::update;
/**
 * !IMPORTANT INFO
自动登录和自动注册.api名称定了.注释描述一下在这里.
 */
#[update]
fn auto_register_user() -> Result<UserProfile, String> {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let caller = caller();
    if caller == Principal::anonymous() {
      return Err(String::from("AnonymousNotAllowRegistering"));
    }
    // init a defautl config for user if config not exsit
    ctx.user_service.get_config(&caller);
    match ctx.user_service.get_user(&caller) {
      Some(profile) => Ok(profile),
      None => {
        let user_profile = UserProfile {
          name: "".to_string(),
          owner: caller,
          create_time: time(),
        };
        match ctx.user_service.insert_user(user_profile.clone()) {
          Ok(user_profile) => {
            ctx.id += 1; // 注册成功，id + 1
            Ok(user_profile)
          }
          Err(error) => Err(error),
        }
      }
    }
  })
}

use crate::common::guard::user_owner_guard;
use crate::user::domain::UserProfile;
#[update(guard = "admin_guard")]
fn list_all_user() -> Vec<UserProfile> {
  CONTEXT.with(|c| {
    let context = c.borrow();
    let users = Vec::from_iter(context.user_service.users.values().cloned());
    return users;
  })
}

#[update(guard = "user_owner_guard")]
fn set_user_config(cfg: UserConfig) -> UserConfig {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    ctx.user_service.add_config(&caller(), cfg);
    let data = ctx.user_service.get_config(&caller());
    return data;
  })
}

#[query(guard = "user_owner_guard")]
fn get_user_config() -> UserConfig {
  CONTEXT.with(|c| {
    let mut ctx = c.borrow_mut();
    let data = ctx.user_service.get_config(&caller());
    return data;
  })
}

#[query(guard = "admin_guard")]
fn user_quantity() -> u32 {
  CONTEXT.with(|c| {
    let ctx = c.borrow_mut();
    let num = ctx.user_service.user_quantity();
    return num;
  })
}

//   The replica returned a replica error: Replica Error:
// reject code CanisterError, reject message IC0504:
// Canister v7g7o-oiaaa-aaaag-qcj3q-cai violated contract:
// "ic0_call_new" cannot be executed in non replicated query
// mode, error code Some("IC0504") take about 20s to get
// ret.
#[update(guard = "user_owner_guard")]
async fn get_balance() -> u64 {
  let balance = check_callers_balance().await.e8s();
  return balance;
}

use ic_cdk::api::caller;
use ic_ledger_types::{
  account_balance, AccountBalanceArgs, AccountIdentifier, Tokens,
  DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

async fn check_callers_balance() -> Tokens {
  account_balance(
    MAINNET_LEDGER_CANISTER_ID,
    AccountBalanceArgs {
      account: AccountIdentifier::new(&caller(), &DEFAULT_SUBACCOUNT),
    },
  )
  .await
  .expect("call to ledger failed")
}

#[cfg(test)] //unit-test attribute(#) in rust syntax
mod tests {

  use super::*;

  #[test]
  fn test_account_id_from_hex() {
    //TODO :  hex should be  (0-9, a-f or A-F). but . the
    // above is not . or it is ? some how transform? let
    // p = String::from("
    // b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae");
    let p = String::from(
      "rintb-5nazg-thqf4-rnq2c-6geuh-ufcjx-fsfm7-qinyq-ma2gb-5rgny-7ae",
    );
    // len should be 56 or 64 (56 is 64-8  <8 - symbol in
    // total? > )
    let _len = p.len();
    let account = AccountIdentifier::new(
      &Principal::from_text(p).unwrap(),
      &DEFAULT_SUBACCOUNT,
    );
    let addr = account.to_hex();
    println!("addr is {}", addr);
    assert!(false); //:   ! means macro here. ! is not "not"
                    //: here ...  silly
                    //: syntax.
  }
}
