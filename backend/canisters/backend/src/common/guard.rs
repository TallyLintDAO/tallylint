use ic_cdk::caller;

use crate::STATE;

use super::constants::{GOVERNANCE_BTWL, GOVERNANCE_ZHOU};

#[allow(dead_code)]
pub fn has_user_guard() -> Result<(), String> {
  STATE.with(|c| {
    let ctx = c.borrow();
    let caller = caller();
    ctx
      .user_service
      .get_user(&caller)
      .map(|_| ())
      .ok_or_else(|| String::from("UserNotFound"))
  })
}

pub fn user_owner_guard() -> Result<(), String> {
  STATE.with(|c| {
    let ctx = c.borrow();
    let caller = caller();
    if ctx.user_service.is_owner(&caller) {
      Ok(())
    } else {
      let error_message = format!(
        "Highly maybe not register yet! Caller: {:?} is not the owner,\n
                and 2vxsx-fae is anoymous user ",
        caller.to_string()
      );
      Err(error_message)
    }
  })
}

pub fn admin_guard() -> Result<(), String> {
  let caller = caller().to_string();
  if GOVERNANCE_BTWL == caller || GOVERNANCE_ZHOU == caller {
    Ok(())
  } else {
    Err("caller not admin ".to_string())
  }
}
