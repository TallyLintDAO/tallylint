use candid::Principal;
use pocket_ic::PocketIc;
use std::path::PathBuf;
use std::time::SystemTime;
pub type TimestampMillis = u64;

pub fn principal_to_username(principal: Principal) -> String {
  principal.to_string()[0..5].to_string()
}

pub fn tick_many(env: &mut PocketIc, count: usize) {
  for _ in 0..count {
    env.tick();
  }
}

pub fn now_millis(env: &PocketIc) -> TimestampMillis {
  now_nanos(env) / 1_000_000
}

pub fn now_nanos(env: &PocketIc) -> TimestampMillis {
  env
    .get_time()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_nanos() as u64
}

pub fn local_bin() -> PathBuf {
  let mut file_path = PathBuf::from(
    std::env::var("CARGO_MANIFEST_DIR")
      .expect("Failed to read CARGO_MANIFEST_DIR env variable"),
  );
  file_path.push("wasms");
  file_path
}

#[test]
fn test1() {
  generate_workout(5, 6);
}
fn generate_workout(intensity: u32, random_number: u32) {
  let expensive_closure = |num| {
    println!("calculating slowly ...");
    num
  };

  // Check the conditions before calling the closure
  if intensity > 25 && random_number == 3 {
    println!("Take a break today! Remember to stay hydrated!");
    return; // Return early and skip the closure
  }

  if intensity < 25 {
    println!("Today, do {} pushups!", expensive_closure(intensity));
    println!("Next, do {} situps!", expensive_closure(intensity));
  } else {
    println!("Today, run for {} minutes!", expensive_closure(intensity));
  }
}
