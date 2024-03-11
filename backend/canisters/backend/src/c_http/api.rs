//https://internetcomputer.org/docs/current/developer-docs/gas-cost
pub fn calculate_cost(
  node_num: u32,
  send_bytes: u64,
  receive_bytes: u64,
) -> u128 {
  let base_fee_http_request_init =
    (3_780_000 * node_num) as u128 * node_num as u128;
  let request_cost = 400 * node_num as u128 * send_bytes as u128;
  let response_cost = 800 * node_num as u128 * receive_bytes as u128;
  base_fee_http_request_init + request_cost + response_cost
}
