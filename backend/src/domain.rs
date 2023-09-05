// #[derive(Debug, Clone, CandidType, Deserialize)]
// pub struct UserRegisterCommand {
//     pub email: String,
//     pub name: String,
//     pub memo: String,
// }

// impl UserRegisterCommand {
//     pub fn build_profile(
//         self,
//         id: UserId,
//         owner: Principal,
//         status: UserStatus,
//         created_at: u64,
//     ) -> UserProfile {
//         UserProfile::new(
//             id,
//             owner,
//             self.email,
//             self.name,
//             0,
//             "".to_string(),
//             "".to_string(),
//             vec![],
//             "".to_string(),
//             self.memo,
//             status,
//             created_at,
//         )
//     }
// }
