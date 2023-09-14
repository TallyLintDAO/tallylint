use candid::{CandidType, Deserialize, Principal};

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserProfile {
    pub owner: Principal, // 用户 Principal
    pub name: String,
    pub create_time: u64,
    pub full_wallet_info_array: Vec<FullWalletInfo>, //CustomizedWalletInfo
                                                     // pub custom_wallet_info: Option<CustomWalletInfo>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UserInfo {
    pub owner: Principal, // 用户 Principal
    pub name: String,
    pub create_time: u64,
}

pub fn construct_user_info(user_profile: UserProfile) -> UserInfo {
    UserInfo {
        owner: user_profile.owner,
        name: user_profile.name,
        create_time: user_profile.create_time,
    }
}

// use ic_cdk::api::time;
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct FullWalletInfo {
    // get from frontend input
    pub wallet_info: WalletInfo,
    // backend auto-gen
    pub id: String,
    pub create_time: u64, //ic_cdk::api::time();
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct WalletInfo {
    // frontend para input
    pub address: String,
    pub from: String, //from which wallet_type: such as  NNS Plug  Stoic AstorMe  .. maybe add more
    pub name: String,
}

impl PartialEq for FullWalletInfo {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the addr field of the front_end_wallet_info field
        self.wallet_info.address == other.wallet_info.address
    }
}

impl UserProfile {
    pub fn new(owner: Principal, name: String, created_at: u64) -> Self {
        Self {
            owner,
            name,
            create_time: created_at,
            full_wallet_info_array: Vec::new(),
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
                create_time: 1234567890,
                full_wallet_info_array: vec![
                    FullWalletInfo {
                        wallet_info: WalletInfo {
                            address: "0x1234567890abcdef".to_string(),
                            from: "NNS Plug".to_string(),
                            name: "Alice's Wallet".to_string(),
                        },
                        id: "1234567890".to_string(),
                        create_time: 1234567890,
                    },
                    FullWalletInfo {
                        wallet_info: WalletInfo {
                            address: "0x0987654321fedcba".to_string(),
                            from: "Stoic AstorMe".to_string(),
                            name: "Alice's Second Wallet".to_string(),
                        },
                        id: "0987654321".to_string(),
                        create_time: 1234567890,
                    },
                ],
            },
            UserProfile {
                owner: Principal::from_text(
                    "ck7ij-a5lub-pklz3-xrpmk-hifoi-xikak-va7ss-hxvqo-5paw2-zx2bw-lae",
                )
                .unwrap(),
                name: "Bob".to_string(),
                create_time: 1234567890,
                full_wallet_info_array: vec![
                    FullWalletInfo {
                        wallet_info: WalletInfo {
                            address: "0x1357902468acefdb".to_string(),
                            from: "NNS Plug".to_string(),
                            name: "Bob's Wallet".to_string(),
                        },
                        id: "1357902468".to_string(),
                        create_time: 1234567890,
                    },
                    FullWalletInfo {
                        wallet_info: WalletInfo {
                            address: "0x2468135790bdfeca".to_string(),
                            from: "Stoic AstorMe".to_string(),
                            name: "Bob's Second Wallet".to_string(),
                        },
                        id: "2468135790".to_string(),
                        create_time: 1234567890,
                    },
                ],
            },
        ];

        println!("{:#?}", user_profiles);
        // assert_eq!(1,2);
    }
}
