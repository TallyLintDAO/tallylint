use time::OffsetDateTime;
pub fn timestamp_to_date(timestamp_ns: u64) -> String {
  let timestamp_s = timestamp_ns / 1_000_000_000;
  let date = OffsetDateTime::from_unix_timestamp(timestamp_s as i64).unwrap();
  format!(
    "{:04}_{:02}_{:02}_{:02}_{:02}_{:02}",
    date.year(),
    date.month()as u32,
    date.day(),
    date.hour(),
    date.minute(),
    date.second()
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test2() {
    let date = timestamp_to_date(1708236479191334820);
    eprintln!("date is {}", date);
  }
}

// cargo test test_timestamp_to_date -- --nocapture
