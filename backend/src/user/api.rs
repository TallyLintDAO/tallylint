use super::domain::*;
use crate::user::domain::UserRegisterCommand;
use crate::CONTEXT;
use candid::Principal;
use ic_cdk_macros::{query, update};
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
                // let id = ctx.id;
                let now = ctx.env.now();
                let empty_name = "".to_string(); //+ &generate_random_string(6);
                let cmd = UserRegisterCommand {
                    // email: "".to_string(),
                    name: empty_name,
                    // memo: "".to_string(),
                };
                let user = cmd.build_profile(caller, now);
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

const MAX_WALLET_NAME_LENGTH: usize = 64;
#[allow(dead_code)]
// #[query]
fn test_print() -> u32 {

    ic_cdk::println!("test_print");
    return 0;
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

#[query]
fn user_quantity() -> u32 {
    CONTEXT.with(|c| {
        let ctx = c.borrow_mut();
        let num = ctx.user_service.user_quantity();
        let _test = 1;
        return num;
    })
}

/**
插入,和更新钱包.
输入:钱包的Principle
输出:更新是否成功
*/
// test ok
use crate::common::guard::user_owner_guard;
#[update(guard = "user_owner_guard")]
// #[update]
fn add_wallet(front_end_wallet_info: FrontEndWalletInfo) -> Result<bool, String> {
    if front_end_wallet_info.name.len() > MAX_WALLET_NAME_LENGTH {
        return Err(String::from("Wallet name exceeds maximum length 64"));
    }
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        let principal_str = user.to_text().to_string();
        ic_cdk::println!("{:?}", principal_str);
        let mut custom_wallet_info = CustomWalletInfo {
            front_end_wallet_info: front_end_wallet_info.clone(),
            id: "id".to_string() + &front_end_wallet_info.addr.to_string(),
            register_time: ic_cdk::api::time(),
        };
        custom_wallet_info.id =
            "wallet_".to_string() + &custom_wallet_info.front_end_wallet_info.addr.to_string();
        let _msg = ctx
            .user_service
            .add_wallet(&user, custom_wallet_info)
            .ok_or("cant add".to_string());
        return Ok(true);
    })
}

// #[update(guard = "user_owner_guard")]
#[update]
fn delete_wallet(wallet_addr: String) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        ctx.user_service
            .delete_wallet(&user, wallet_addr)
            .ok_or(String::from("WalletNotFound"))
    })
}

// test ok
// #[update(guard = "user_owner_guard")]
#[update]
fn query_wallet_array() -> Result<Vec<CustomWalletInfo>, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        ctx.user_service
            .query_wallet_array(&user)
            .ok_or(String::from("WalletNotFound"))
    })
}

use crate::user::domain::UserProfile;
// #[update(guard = "user_owner_guard")]
#[update]
fn list_all_user() -> Vec<UserProfile> {
    CONTEXT.with(|c| {
        let context = c.borrow();
        let users = Vec::from_iter(context.user_service.users.values().cloned());
        return users;
    })
}
