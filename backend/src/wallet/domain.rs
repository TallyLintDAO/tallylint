pub(crate) use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletProfile {
    // primary key
    pub id: u64,
    pub holder: Principal,

    // frontend para input
    pub address: String, // immutable.
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
    
    // backend auto-gen
    pub create_time: u64, //ic_cdk::api::time();

    pub transactions:u64,
    pub last_sync_time:u64,
    pub last_transaction_time:u64,
}


#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletAddCommand {
    pub address: String,
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct  WalletUpdateCommand{
    pub address: String,
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,

    pub transactions:u64,
    pub last_sync_time:u64,
    pub last_transaction_time:u64,
}
