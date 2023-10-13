use ic_cdk::api::management_canister::main::{create_canister, install_code};
use ic_cdk_macros::update;

use super::service::{install_config, new_canister_args};

#[update]
pub async fn create_and_install() -> String {
    let cycles = 4_00_000_000_000; //4TC
    let res = create_canister(new_canister_args(), cycles).await;
    if res.is_err() {
       
        return String::from("create canister error");
    }
    //  CallResult<(CanisterIdRecord,)> : Result----unwrap  ()----tuple .0 to get the first tuple.
    let new_canister_principal = res.unwrap().0.canister_id;
    let result = install_code(install_config(new_canister_principal)).await;
    if result.is_err() {
        let _info=result.map_err(|message| format!(" message {}", message.1)).unwrap_err();
        let error1=String::from("install_code error: ");
        /**
         * ("install_code error: message Only the controllers of the canister bw4dl-smaaa-aaaaa-qaacq-cai can control it.
Canister's controllers: 2vxsx-fae
Sender's ID: bkyz2-fmaaa-aaaaa-qaaaq-cai")
         */
        return error1+&_info;
    }
    return String::from("create_and_install succuess");
}
