use candid::Principal;
use ic_cdk_macros::{update};

use super::{domain::*};


use crate::CONTEXT;

#[update]
fn register_user() -> Result<String, String> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();

        if caller == Principal::anonymous() {
            return Err(String::from("AnonymousNotAllowRegistering") );
        }

        let now = ctx.env.now();





        
        
        
        // fn generate_random_string(length: usize, rng: &mut ThreadRng) -> String {
        //   let mut string = String::with_capacity(length);
        //   let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        
        //   for _ in 0..length {
        //     let random_char = chars.chars().nth(rng.gen_range(0..chars.len())).expect("Invalid char");
        //     string.push(random_char);
        //   }
        
        //   string
        // }
        // let mut cmd = UserRegisterCommand {
        //     email: generate_random_string(12, &mut rng),
        //     name: generate_random_string(12, &mut rng),
        //     memo: generate_random_string(12, &mut rng),
        // };
        let cmd = UserRegisterCommand {
            email: "test_email@example.com".to_string(),
            name: "Test User".to_string(), 
            memo: "For testing".to_string()
        };


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


