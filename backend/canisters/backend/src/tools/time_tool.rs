use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

// TODO  this impl work good on amd64 arch. not ok in wasm vm.
pub fn timestamp_to_date(timestamp_ns: u64) -> DateTime<Utc> {
  let timestamp_s = timestamp_ns / 1_000_000_000; // convert to seconds
  let nanos = (timestamp_ns % 1_000_000_000) as u32; // get remaining nanoseconds
  let duration = Duration::new(timestamp_s, nanos);
  let datetime = UNIX_EPOCH + duration;
  DateTime::<Utc>::from(datetime)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_timestamp_to_date() {
    let timestamp_ns = 1708236479191334820;
    let date = timestamp_to_date(timestamp_ns);
    println!("{}", date);
    assert_eq!(date.to_string(), "2024-02-18 10:14:39.191334820 UTC");
  }
}

// cargo test test_timestamp_to_date -- --nocapture
