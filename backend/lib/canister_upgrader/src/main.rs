use ic_utils::call::AsyncCall;
use ic_utils::interfaces::management_canister::builders::InstallMode;
use ic_utils::interfaces::management_canister::CanisterStatus;
use ic_utils::interfaces::ManagementCanister;
use std::fs::read;
use std::io::Error;
use candid::{encode_one, Principal};
use canister_agent_utils::{build_ic_agent, get_canister_wasm, get_dfx_identity, CanisterName};


#[tokio::main]
async fn main(){
    install().await;
}
async fn install(){
    // FIXME this local  url says not valid. but i can run `dfx deploy xx`in any new terminal.
    let url_local=String::from("127.0.0.1:40010");
    
    let url_ic=String::from("https://ic0.app/");
    let controller=String::from("btwlz");

    // INFO this need use input passwd in terminal if have passwd.
    let identity = get_dfx_identity(&controller);
    let agent = build_ic_agent(url_local, identity).await;
    let management_canister = ManagementCanister::create(&agent);

    // INFO this is local or ic canister_id
    let canister_id=Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap();
    // FIXME how to achive this goal ?
    let args= "skip_pre_upgrade = true" ;

    let wasm_file_path = "/home/btwl/code/ic/tax_lint/target/wasm32-unknown-unknown/release/backend.wasm";
    let wasm_bytes = read(wasm_file_path).expect("file not exsit");
        
        match management_canister
            .install_code(&canister_id, &wasm_bytes)
            .with_mode(InstallMode::Upgrade)
            .with_arg(args)
            .call_and_wait()
            .await
        {
            Ok(_) => println!("Wasm upgraded"),
            Err(error) => println!("Upgrade failed: {error:?}"),
        };
}

// ic_utils lib hot fix patch : git commit: b74445e1da0a6afefc3a08372f74e8ea416cd1ba