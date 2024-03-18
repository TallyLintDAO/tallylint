use std::fmt::{Display, Formatter};

use candid::Principal;

use rand::{
  distributions::{Distribution, Standard},
  random, Rng, RngCore,
};
pub struct MessageId(u128);

impl Distribution<MessageId> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MessageId {
    MessageId(rng.gen())
  }
}

impl From<u128> for MessageId {
  fn from(value: u128) -> MessageId {
    MessageId(value)
  }
}

impl Display for MessageId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.0, f)
  }
}

use super::setup::NNS_INTERNET_IDENTITY_CANISTER_ID;

pub fn random_principal() -> Principal {
  let random_bytes = rand::thread_rng().next_u32().to_ne_bytes();

  Principal::from_slice(&random_bytes)
}

pub fn random_user_principal() -> (Principal, Vec<u8>) {
  let algorithm_bytes = [
    48u8, 60, 48, 12, 6, 10, 43, 6, 1, 4, 1, 131, 184, 67, 1, 2, 3, 44, 0,
  ];

  // TODO : productivtiy .
  // if user need this rand dep . what cargo just auto import for me ?? like
  // JAVA
  let random_bytes: [u8; 32] = random();

  let mut public_key = Vec::from(algorithm_bytes);
  public_key.push(NNS_INTERNET_IDENTITY_CANISTER_ID.as_slice().len() as u8);
  public_key.extend_from_slice(NNS_INTERNET_IDENTITY_CANISTER_ID.as_slice());
  public_key.extend_from_slice(&random_bytes);

  (Principal::self_authenticating(&public_key), public_key)
}

pub fn random_string() -> String {
  rand::thread_rng().next_u32().to_string()
}

pub fn random_message_id() -> MessageId {
  random()
}
