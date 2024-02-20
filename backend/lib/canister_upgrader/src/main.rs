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
    // TODO what is local replica run by dfx start ?
    let url=String::from("xx.com");
    let controller=String::from("btwlz");
    let identity = get_dfx_identity(&controller);
    let agent = build_ic_agent(url, identity).await;
    let management_canister = ManagementCanister::create(&agent);

    // TODO this is local or ic canister_id
    let canister_id=Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap();
    // TODO how to achive this goal ?
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