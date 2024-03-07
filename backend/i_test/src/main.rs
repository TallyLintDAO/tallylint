use client::setup::setup_new_env;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
pub mod backend_test;
pub mod client;


fn main(){
  setup_new_env(None);

}
