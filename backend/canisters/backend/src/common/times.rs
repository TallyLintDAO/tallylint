pub fn timestamp_ms_float_to_ms_u64(milliseconds: f64) -> u64 {
  milliseconds.trunc() as u64
  // dropping everything after the decimal point,
  // dont need that level of precise
}
