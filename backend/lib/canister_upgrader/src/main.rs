use ic_utils::call::AsyncCall;
use ic_utils::interfaces::management_canister::builders::InstallMode;
use ic_utils::interfaces::management_canister::CanisterStatus;
use ic_utils::interfaces::ManagementCanister;


#[tokio::main]
async fn main(){
    install().await;
}
async fn install(){
    let arg= "skip_pre_upgrade" ;
    let canister_id="111";
    // /home/btwl/code/ic/tax_lint/target/wasm32-unknown-unknown/release/backend.wasm
    let wasm_bytes=Vec::new();
        match management_canister
            .install_code(canister_id, wasm_bytes)
            .with_mode(InstallMode::Upgrade)
            .with_arg(args)
            .call_and_wait()
            .await
        {
            Ok(_) => println!("Wasm upgraded"),
            Err(error) => println!("Upgrade failed: {error:?}"),
        };
}