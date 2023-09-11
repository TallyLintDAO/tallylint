use candid::Principal;
use ic_cdk_macros::update;

use super::domain::*;
use crate::CONTEXT;

use crate::user::domain::UserRegisterCommand;
#[update]
fn register_user(cmd: UserRegisterCommand) -> Result<String, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();

        if caller == Principal::anonymous() {
            return Err(String::from("Require  principle , AnonymousNotAllowRegistering") );
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

#[update]
fn auto_register_user() -> Result<UserProfile, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        if caller == Principal::anonymous() {
            return Err(String::from("AnonymousNotAllowRegistering"));
        }

        match ctx.user_service.get_user(&caller) {
            Some(u) => Ok(u),
            None => {
                let id = ctx.id;
                let now = ctx.env.now();
                let cmd = UserRegisterCommand {
                    email: "".to_string(),
                    name: "".to_string(),
                    memo: "".to_string(),
                };

                let user = cmd.build_profile(id, caller, UserStatus::Enable, now);

                match ctx.user_service.insert_user(user.clone()) {
                    Ok(_) => {
                        ctx.id += 1; // 注册成功，id + 1
                        Ok(user)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    })
}

