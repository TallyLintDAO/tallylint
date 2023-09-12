use candid::Principal;
use ic_cdk_macros::update;
// use random_string::generate;
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
            return Err(String::from(
                "Require  principle , AnonymousNotAllowRegistering",
            ));
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

/**
后端设置用户字段:  
principalID  
名字  
创建账号的时间  
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
            Some(u) => Ok(u),
            None => {
                let id = ctx.id;
                let now = ctx.env.now();
                let random_user_name= "user".to_string()+&generate_random_string(6);
                let cmd = UserRegisterCommand {
                    // email: "".to_string(),
                    name: random_user_name,
                    // memo: "".to_string(),
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

const MAX_WALLET_NAME_LENGTH: usize = 32;

/**
 插入,和更新钱包.
 输入:钱包的Principle
 输出:更新是否成功
 */
use crate::common::guard::user_owner_guard;
#[update(guard = "user_owner_guard")]
fn update_wallet(info:CustomWalletInfo) -> Result<bool, String> {
    if info.wallet_name.len() > MAX_WALLET_NAME_LENGTH {
        return Err(String::from("Wallet name exceeds maximum length 32"));
    }

    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();

        ctx.user_service
            .update_wallet(&user, info)
            .ok_or(String::from("UserNotFound"))
    })
}








use rand::Rng;

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let charset_length = CHARSET.len();

    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| {
            let random_index = rng.gen_range(0, charset_length);
            CHARSET[random_index] as char
        })
        .collect();

    return random_string;
}

