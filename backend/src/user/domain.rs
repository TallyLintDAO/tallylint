use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserProfile {
    pub owner: Principal, // 用户 Principal
    pub name: String,
    pub created_at: u64,
    // pub custom_wallet_info_array: Vec<CustomWalletInfo>,
    pub custom_wallet_info: Option<CustomWalletInfo>,
}

impl UserProfile {
    pub fn new(
        owner: Principal,
        name: String,
        created_at: u64,
    ) -> Self {
        Self {
            owner,
            name,
            created_at,
            custom_wallet_info:None,
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum UserStatus {
    Enable,
    Disable,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserRegisterCommand {
    // pub email: String,
    pub name: String,
    // pub memo: String,
}

impl UserRegisterCommand {
    pub fn build_profile(
        self,
        owner: Principal,
        created_at: u64,
    ) -> UserProfile {
        UserProfile::new(
            owner,
            self.name,
            created_at,
        )
    }
}


#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserWalletUpdateCommand {
    pub user: Principal,
    pub wallet: Principal,
}

// use ic_cdk::api::time;
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CustomWalletInfo {
    // frontend para input
    pub wallet_addr: Principal,
    pub wallet_type: String,
    pub wallet_name: String, 
    // backend auto-gen
    pub wallet_id: String,
    pub wallet_register_time: u64,//ic_cdk::api::time();
}

