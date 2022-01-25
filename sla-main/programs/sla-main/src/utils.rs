use anchor_lang::prelude::*;
use anchor_spl;

pub fn check_token_owner(mint: &Pubkey, user: &Pubkey, token_account: &AccountInfo) -> bool {
  let token_account_pda = anchor_spl::associated_token::get_associated_token_address(user, mint);

  let exists = token_account_pda == token_account_pda.key();
  let holds = anchor_spl::token::accessor::amount(&token_account).unwrap() == 1;

  exists && holds
}
