
pub fn can_hay_be_minted(current_timestamp: i64, last_mint: i64) -> bool {
  // Returns True if at least 23 hours have elapsed since the last mint
  current_timestamp >= last_mint + 23 * 60 * 60
}