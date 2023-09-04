use candid::Principal;
use ic_cdk_macros::{query, update};

use super::{domain::*, error::UserError};

use crate::common::guard::user_owner_guard;
use crate::context::DaoContext;
use crate::CONTEXT;

#[update]
fn register_user(cmd: UserRegisterCommand) -> Result<String, UserError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();

        if caller == Principal::anonymous() {
            return Err(UserError::AnonymousNotAllowRegistering);
        }

        let now = ctx.env.now();
        let user = cmd.build_profile(id, caller, UserStatus::Enable, now);

        match ctx.user_service.insert_user(user) {
            Ok(p) => {
                ctx.id += 1; // 注册成功，id + 1
                Ok(p.to_string())
            }
            Err(e) => Err(e),
        }
    })
}
