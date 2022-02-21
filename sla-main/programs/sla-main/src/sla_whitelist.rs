use anchor_lang::prelude::*;
use std::str::FromStr;

const SKIN_WHITELIST_TOKEN: &str = "HF2uth4uW4kaj93oXQdsCavrQ4wP1GfNqsVky7Fu6Gjx";
const CLOTHING_WHITELIST_TOKEN: &str = "6Xz1MpEuQQbFpRWEdyLz5oNbHdYQ1KvpyADRu5Qnw6dD";
const EYES_WHITELIST_TOKEN: &str = "DLkpd8L37Wi62aSbxVToWLMpDw9E6tw7skdqDZUQu4pq";
const HAT_WHITELIST_TOKEN: &str = "8wViTotiM8PiYxWCUCRTioC4QaBWvuCKer96tYmttCGe";
const MOUTH_WHITELIST_TOKEN: &str = "E9mZcxwRN3qNpDByVVCySWKb14jTZWRJrYHTH7125H4J";

pub fn check_whitelist_mint_id(mint: &Pubkey, trait_id: u8) -> bool {
  
  let target = match trait_id {
    2 => SKIN_WHITELIST_TOKEN,
    3 => CLOTHING_WHITELIST_TOKEN,
    4 => EYES_WHITELIST_TOKEN,
    5 => HAT_WHITELIST_TOKEN,
    6 => MOUTH_WHITELIST_TOKEN,
    _ => "",
  };

  &Pubkey::from_str(target).unwrap() == mint
}