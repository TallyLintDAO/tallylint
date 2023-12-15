use chrono::format::strftime::StrftimeItems;
use pocket_ic::*;
use std::time::SystemTime;
fn main() {
  // println!("Hello, world!");
  let pic = PocketIc::new();
  use chrono::{DateTime, Utc};
  let t: SystemTime = pic.get_time();
  let datetime: DateTime<Utc> = t.into();
  println!("{}", datetime.to_string());
}

