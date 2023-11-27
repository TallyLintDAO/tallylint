/// 把指定天数转为纳秒(ns)
#[allow(dead_code)]
pub fn days_to_ns(days: u64) -> u64 {
  days as u64 * 86_400_000_000_000
  // 24 * 60 * 60 * 1000 * 1000000
}
