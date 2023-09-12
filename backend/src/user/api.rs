use candid::Principal;
use ic_cdk_macros::update;
// use random_string::generate;
use super::domain::*;
use crate::user::domain::UserRegisterCommand;
use crate::CONTEXT;
// #[update]
// fn register_user(cmd: UserRegisterCommand) -> Result<String, String> {
//     CONTEXT.with(|c| {
//         let mut ctx = c.borrow_mut();
//         let id = ctx.id;
//         let caller = ctx.env.caller();

//         if caller == Principal::anonymous() {
//             return Err(String::from(
//                 "Require  principle , AnonymousNotAllowRegistering",
//             ));
//         }
//         let now = ctx.env.now();
//         let user = cmd.build_profile(id, caller, UserStatus::Enable, now);
//         match ctx.user_service.insert_user(user) {
//             Ok(p) => {
//                 ctx.id += 1; // 注册成功，id + 1
//                 Ok(p.to_string())
//             }
//             Err(e) => Err(e),
//         }
//     })
// }
// use crate::common::tool::generate_random_string;
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
                let random_user_name = "".to_string() ;//+ &generate_random_string(6);
                let cmd = UserRegisterCommand {
                    // email: "".to_string(),
                    name: random_user_name,
                    // memo: "".to_string(),
                };
                let user = cmd.build_profile( caller, now);
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
fn update_wallet(mut info: CustomWalletInfo) -> Result<bool, String> {
    if info.wallet_name.len() > MAX_WALLET_NAME_LENGTH {
        return Err(String::from("Wallet name exceeds maximum length 32"));
    }
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        info.wallet_id = ctx.id.to_string() + "_wallet_" ;
        ctx.id += 1;//+ &generate_random_string(6);
        info.wallet_id=info.wallet_id+&ctx.id.to_string();
        info.wallet_register_time=ic_cdk::api::time();
        ctx.user_service
            .update_wallet(&user, info)
            .ok_or(String::from("UserNotFound"))
    })
}

#[update(guard = "user_owner_guard")]
fn delete_wallet(wallet_addr:String) -> Result<bool, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let user = ctx.env.caller();
        ctx.user_service
            .delete_wallet(&user)
            .ok_or(String::from("UserNotFound"))
    })
}


// #[cfg(test)]
// mod tests {
//     use crate::user::domain::CustomWalletInfo;
//     use candid::Principal;

//     use super::update_wallet;

//     #[test]
//     fn test_wallet() {
//         let info = CustomWalletInfo {
//             wallet_addr: Principal::from_text("b76rz-axcfs-swjig-bzzpx-yt5g7-2vcpg-wmb7i-2mz7s-upd4f-mag4c-yae").unwrap(),
//             wallet_type: "Ledger".to_string(),
//             wallet_name: "My Ledger Wallet".to_string(),  
//             wallet_id: "123".to_string(),
//             wallet_register_time: ic_cdk::api::time()
//           };
//         let res= update_wallet(info)  ;
//         println!("{:?}",res);
//         assert_eq!(0,1);
//     }
// }
