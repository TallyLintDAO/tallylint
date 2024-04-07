/// 把指定天数转为纳秒(ns)
#[allow(dead_code)]
pub fn days_to_ns(days: u64) -> u64 {
  days as u64 * 86_400_000_000_000
  // 24 * 60 * 60 * 1000 * 1000000
}

// Function to convert milliseconds to nanoseconds
pub fn timestamp_ms_float_to_ns(milliseconds: f64) -> u64 {
  (milliseconds * 1_000_000.0) as u64
}

pub fn timestamp_ms_float_to_ms_u64(milliseconds: f64) -> u64 {
  milliseconds.trunc() as u64
  // dropping everything after the decimal point,
  // dont need that level of precise
}

// Function to convert nanoseconds to milliseconds
pub fn timestamp_ns_to_ms_float(nanoseconds: u64) -> f64 {
  nanoseconds as f64 / 1_000_000.0
}
