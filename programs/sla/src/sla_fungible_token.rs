use std::fmt;
use anchor_lang::prelude::*;

use crate::{sla_constants, utils, sla_token};


#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum FungibleAsset {
  ID_CARD = 1,
  BADGE_BRONZE = 2,
  BADGE_SILVER = 3,
  BADGE_GOLD = 4,
  BADGE_PLATINUM = 5,
  BADGE_DIAMOND = 6,
}

impl FungibleAsset {
  pub fn from_u8(value: u8) -> FungibleAsset {
    match value {
      1 => FungibleAsset::ID_CARD,
      2 => FungibleAsset::BADGE_BRONZE,
      3 => FungibleAsset::BADGE_SILVER,
      4 => FungibleAsset::BADGE_GOLD,
      5 => FungibleAsset::BADGE_PLATINUM,
      6 => FungibleAsset::BADGE_DIAMOND,
      _ => panic!("Unknown value: {}", value),
    }
  }

  pub fn get_mint(&self) -> &str {
    match self {
      FungibleAsset::ID_CARD => sla_constants::ID_CARD_MINT,
      FungibleAsset::BADGE_BRONZE => sla_constants::BADGE_BRONZE_MINT,
      FungibleAsset::BADGE_SILVER => sla_constants::BADGE_SILVER_MINT,
      FungibleAsset::BADGE_GOLD => sla_constants::BADGE_GOLD_MINT,
      FungibleAsset::BADGE_PLATINUM => sla_constants::BADGE_PLATINUM_MINT,
      FungibleAsset::BADGE_DIAMOND => sla_constants::BADGE_DIAMOND_MINT,
    }
  }

  pub fn get_price(&self) -> u16 {
    match self {
      FungibleAsset::ID_CARD => sla_constants::PRICE_ID_CARD,
      FungibleAsset::BADGE_BRONZE => sla_constants::PRICE_BADGE_BRONZE,
      FungibleAsset::BADGE_SILVER => sla_constants::PRICE_BADGE_SILVER,
      FungibleAsset::BADGE_GOLD => sla_constants::PRICE_BADGE_GOLD,
      FungibleAsset::BADGE_PLATINUM => sla_constants::PRICE_BADGE_PLATINUM,
      FungibleAsset::BADGE_DIAMOND => sla_constants::PRICE_BADGE_DIAMOND,
    }
  }

  pub fn get_max_supply(&self) -> u64 {
    let supply = match self {
      FungibleAsset::BADGE_BRONZE => sla_constants::SUPPLY_BADGE_BRONZE,
      FungibleAsset::BADGE_SILVER => sla_constants::SUPPLY_BADGE_SILVER,
      FungibleAsset::BADGE_GOLD => sla_constants::SUPPLY_BADGE_GOLD,
      FungibleAsset::BADGE_PLATINUM => sla_constants::SUPPLY_BADGE_PLATINUM,
      FungibleAsset::BADGE_DIAMOND => sla_constants::SUPPLY_BADGE_DIAMOND,
      _ => 0,
    };
    u64::from(supply)
  }

  pub fn is_badge(&self) -> bool {
    match &self {
      FungibleAsset::BADGE_BRONZE | 
      FungibleAsset::BADGE_SILVER |
      FungibleAsset::BADGE_GOLD |
      FungibleAsset::BADGE_PLATINUM |
      FungibleAsset::BADGE_DIAMOND => true,
      _ => false
    }
  }
}

impl fmt::Display for FungibleAsset {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
          FungibleAsset::ID_CARD => write!(f, "ID Card"),
          FungibleAsset::BADGE_BRONZE => write!(f, "Bronze Badge"),
          FungibleAsset::BADGE_SILVER => write!(f, "Silver Badge"),
          FungibleAsset::BADGE_GOLD => write!(f, "Gold Badge"),
          FungibleAsset::BADGE_PLATINUM => write!(f, "Platinum Badge"),
          FungibleAsset::BADGE_DIAMOND => write!(f, "Diamond Badge"),
      }
  }
}


pub fn assert_mint_address<'info>(mint_given: &Pubkey, asset_id: u8) -> bool {
  let expected = FungibleAsset::from_u8(asset_id);
  utils::assert_address(mint_given, expected.get_mint())
}


pub fn mint_fungible_asset<'info>(
  mint: AccountInfo<'info>,
  ata: AccountInfo<'info>,
  user: AccountInfo<'info>,
  treasury: AccountInfo<'info>,
  hay_user_ata: AccountInfo<'info>,
  hay_treasury_ata: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  fungible_asset: FungibleAsset,
  treasury_bump: u8,
) -> ProgramResult {

  let signer_seeds = &[&[sla_constants::PREFIX_TREASURY.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];
  let price = fungible_asset.get_price();

  msg!("Minting a new {}", fungible_asset.to_string());
  sla_token::mint_tokens(mint, ata, treasury, token_program.clone(), Some(signer_seeds), 1)?;

  msg!("Transferring {} $HAY to treasury", price);
  sla_token::transfer_tokens(
    hay_user_ata, 
    hay_treasury_ata, 
    user,
    token_program, 
    u64::from(price),
  )?;
  
  Ok(())
}