use candid::Decode;
use candid::Principal;

// usage :
// https://forum.dfinity.org/t/how-do-you-geeks-use-ic-agent-in-rust-for-testing/24593
// todo: offical dont have tutorial of using this . maybe can PR some to the
// offical doc.
#[tokio::test]
async fn test1() {
  // Import the ic-agent crate and the candid crate
  use candid::{CandidType, Decode, Encode};
  use ic_agent::Agent;

  // Define a struct that corresponds to the Candid record type of the
  // add_wallet argument
  #[derive(CandidType)]
  struct AddWalletArgument {
    address: String,
    name: String,
    from: String,
  }
  // ! simulate call from rust code :  dfx canister call backend add_wallet
  // '(record { address = "a1"; name = "AmydaLu"; from = "asdaw" })' Create an
  // instance of the Agent struct
  let agent = Agent::builder()
    // Use the URL of the Internet Computer or a local replica
    .with_url("http://localhost:8000/")
    // ! pgrep replica  # get pid from name
    // !  lsof -i -P -n | grep LISTEN | grep <pid>
    //  todo ! which port should i use ?
    // Use an identity to sign messages
    .with_identity(create_identity())
    // todo: how to simulate : dfx use btwl1 ? and the account have passwd. when
    // i exec dfx cmd . it requires my passwd in bash
    .build()?;

  // Fetch the root key of the replica
  agent.fetch_root_key().await?;

  use backend::common::constants;
  // Get the canister identifier of the backend canister
  let backend_canister_id =
    Principal::from_text(constants::BACKEND_CANISTER_ID.to_string())?;
  let test_addr =
    "67cff347f803890e6bd1865b5a116c8c06384992b93d2fc8bb8da2f22fffc6a3"
      .to_string();

  // Create a Candid value that represents the argument of the add_wallet method
  let argument = Encode!(&AddWalletArgument {
    address: test_addr,
    name: "AmydaLu_wallet".to_string(),
    from: "nns".to_string(),
  })?;

  // Call the add_wallet method on the backend canister
  let response = agent
    .update(&backend_canister_id, "add_wallet")
    // Pass the Candid value as the argument of the add_wallet method
    .with_arg(argument)
    // Wait for the result of the update call
    .call_and_wait()
    .await?;

  // Decode the result of the update call

  let result = Decode!(response.as_slice(), Result<bool, String>)?;

  match result {
    Ok(_) => println!("Wallet added successfully"),
    Err(msg) => println!("Error: {}", msg),
  }
  // Print the result
  println!("{:?}", result);
}
