use crate::client::setup::setup_new_env;

// #![cfg(test)]
pub mod backend_test;
pub mod client;
// mod ic_replica_server;
fn main() {
  // create_a_nns_wallet();
  setup_new_env();
  println!("Hello, world!");
}
