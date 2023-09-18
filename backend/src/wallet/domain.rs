pub(crate) use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletProfile {
    // primary key
    pub id: u64,
    pub holder: Principal,
    // frontend para input
    pub address: String,
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
    // backend auto-gen
    pub create_time: u64, //ic_cdk::api::time();
}

impl WalletProfile {
    pub fn new(id: u64, holder: Principal, address: String, from: String, name: String, create_time: u64) -> Self { Self { id, holder, address, from, name, create_time } }

    
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletAddCommand {
    pub address: String,
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
}

// impl PartialEq for WalletProfile {
//     fn eq(&self, other: &Self) -> bool {
//         // Compare only the addr field of the front_end_wallet_info field
//         self.address == other.address
//     }
// }
