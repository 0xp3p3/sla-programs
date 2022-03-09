use anchor_lang::prelude::*;

use crate::{
  sla_errors::SlaErrors, 
  sla_constants, 
  sla_token::burn_tokens, 
  utils::str_to_pubkey,
};


pub fn pay_for_edition_mint<'info>(
  master_mint: &Pubkey,
  hay_mint: AccountInfo<'info>,
  hay_user_ata: AccountInfo<'info>,
  user: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
) -> Result<(), SlaErrors> {

  let id_card = str_to_pubkey(sla_constants::MASTER_EDITION_ID_CARD);
  let badge_bronze = str_to_pubkey(sla_constants::MASTER_EDITION_BADGE_BRONZE);
  let badge_silver = str_to_pubkey(sla_constants::MASTER_EDITION_BADGE_SILVER);
  let badge_gold = str_to_pubkey(sla_constants::MASTER_EDITION_BADGE_GOLD);
  let badge_platinum = str_to_pubkey(sla_constants::MASTER_EDITION_BADGE_PLATINUM);
  let badge_diamond = str_to_pubkey(sla_constants::MASTER_EDITION_BADGE_DIAMOND);

  // The user pays a different price depending on what is being minted
  let cost = match *master_mint {
    id_card => sla_constants::PRICE_ID_CARD,
    badge_bronze => sla_constants::PRICE_BADGE_BRONZE,
    badge_silver => sla_constants::PRICE_BADGE_SILVER,
    badge_gold => sla_constants::PRICE_BADGE_GOLD,
    badge_platinum => sla_constants::PRICE_BADGE_PLATINUM,
    badge_diamond => sla_constants::PRICE_BADGE_DIAMOND,
    _ => return Err(SlaErrors::MasterEditionNotRecognised),
  };

  // Burn the tokens
  burn_tokens(
    hay_mint,
    hay_user_ata,
    user,
    token_program,
    None,
    cost
  ).unwrap();

  Ok(())
}