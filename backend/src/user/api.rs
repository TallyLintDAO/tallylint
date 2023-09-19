
use crate::CONTEXT;
use candid::Principal;
use ic_cdk_macros::update;
/**
 * IMPORTANT INFO
自动登录和自动注册.api名称定了.注释描述一下在这里.
 */
#[update]
fn auto_register_user() -> Result<UserProfile, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let caller = ctx.env.caller();
        if caller == Principal::anonymous() {
            return Err(String::from("AnonymousNotAllowRegistering"));
        }
        match ctx.user_service.get_user(&caller) {
            Some(profile) => Ok(profile),
            None => {
                let user_profile = UserProfile {
                    name: "".to_string(),
                    owner: caller,
                    create_time: ctx.env.now(),
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
#[update(guard = "user_owner_guard")]
fn list_all_user() -> Vec<UserProfile> {
    CONTEXT.with(|c| {
        let context = c.borrow();
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        return users;
    })
}

#[update(guard = "user_owner_guard")]
fn user_quantity() -> u32 {
    CONTEXT.with(|c| {
        let ctx = c.borrow_mut();
        let num = ctx.user_service.user_quantity();
        let _test = 1;
        return num;
    })
}

#[allow(dead_code)]
// #[query]
pub fn get_caller_principal() -> String {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        let caller = &ctx.env.caller();
        return caller.to_text().to_string();
    })
}
