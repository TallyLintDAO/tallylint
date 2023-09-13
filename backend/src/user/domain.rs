use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserProfile {
    pub owner: Principal, // 用户 Principal
    pub name: String,
    pub created_at: u64,
    pub custom_wallet_info_array: Vec<CustomWalletInfo>,
    // pub custom_wallet_info: Option<CustomWalletInfo>,
}
// use ic_cdk::api::time;
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct CustomWalletInfo {
    pub front_end_wallet_info: FrontEndWalletInfo,
    // backend auto-gen
    pub id: String,
    pub register_time: u64, //ic_cdk::api::time();
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct FrontEndWalletInfo {
    // frontend para input
    pub addr: String,
    pub w_type: String, //NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
}

impl PartialEq for CustomWalletInfo {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the addr field of the front_end_wallet_info field
        self.front_end_wallet_info.addr == other.front_end_wallet_info.addr
    }
}



impl UserProfile {
    pub fn new(owner: Principal, name: String, created_at: u64) -> Self {
        Self {
            owner,
            name,
            created_at,
            custom_wallet_info_array: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserRegisterCommand {
    pub name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserWalletUpdateCommand {
    pub user: Principal,
    pub wallet: Principal,
}
impl UserRegisterCommand {
    pub fn build_profile(self, owner: Principal, created_at: u64) -> UserProfile {
        UserProfile::new(owner, self.name, created_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile() {
        let user_profiles: Vec<UserProfile> = vec![
            UserProfile {
                owner: Principal::from_text(
                    "ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae",
                )
                .unwrap(),
                name: "Alice".to_string(),
                created_at: 1234567890,
                custom_wallet_info_array: vec![
                    CustomWalletInfo {
                        front_end_wallet_info: FrontEndWalletInfo {
                            addr: "0x1234567890abcdef".to_string(),
                            w_type: "NNS Plug".to_string(),
                            name: "Alice's Wallet".to_string(),
                        },
                        id: "1234567890".to_string(),
                        register_time: 1234567890,
                    },
                    CustomWalletInfo {
                        front_end_wallet_info: FrontEndWalletInfo {
                            addr: "0x0987654321fedcba".to_string(),
                            w_type: "Stoic AstorMe".to_string(),
                            name: "Alice's Second Wallet".to_string(),
                        },
                        id: "0987654321".to_string(),
                        register_time: 1234567890,
                    },
                ],
            },
            UserProfile {
                owner: Principal::from_text(
                    "ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae",
                )
                .unwrap(),
                name: "Bob".to_string(),
                created_at: 1234567890,
                custom_wallet_info_array: vec![
                    CustomWalletInfo {
                        front_end_wallet_info: FrontEndWalletInfo {
                            addr: "0x1357902468acefdb".to_string(),
                            w_type: "NNS Plug".to_string(),
                            name: "Bob's Wallet".to_string(),
                        },
                        id: "1357902468".to_string(),
                        register_time: 1234567890,
                    },
                    CustomWalletInfo {
                        front_end_wallet_info: FrontEndWalletInfo {
                            addr: "0x2468135790bdfeca".to_string(),
                            w_type: "Stoic AstorMe".to_string(),
                            name: "Bob's Second Wallet".to_string(),
                        },
                        id: "2468135790".to_string(),
                        register_time: 1234567890,
                    },
                ],
            },
        ];

        println!("{:#?}", user_profiles);
        // assert_eq!(1,2);
    }
}
