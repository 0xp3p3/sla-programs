use anchor_lang::prelude::*;
use std::fmt;

use crate::{SlaErrors, sla_fungible_token::FungibleAsset, sla_constants};

const DISCRIMINATOR_LENGTH: usize = 8;

#[account]
#[derive(Copy, Default)]
pub struct AvatarAccount {
  pub traits: Option<AvatarData>,
}

impl AvatarAccount {
  pub const LEN: usize = DISCRIMINATOR_LENGTH + 2 * AvatarData::LEN;

  pub fn init(&mut self) -> Result<(), SlaErrors> {
    match self.traits {
      None => {
        self.traits = Some(AvatarData::init());
        Ok(())
      }
      Some(_) => Err(SlaErrors::AvatarAlreadyInitialized),
    }
  }

  pub fn merge(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    match &mut self.traits {
      Some(traits) => traits.merge(trait_id),
      None => Err(SlaErrors::AvatarPDANotInitialized),
    }
  }
}

#[derive(Clone, Copy, Default, AnchorSerialize, AnchorDeserialize)]
pub struct AvatarData {
  pub skin: bool,
  pub clothing: bool,
  pub eyes: bool,
  pub hat: bool,
  pub mouth: bool,
}

impl AvatarData {
  const LEN: usize = 5;

  pub fn init() -> Self {
    AvatarData {
      skin: false,
      clothing: false,
      eyes: false,
      hat: false,
      mouth: false,
    }
  }

  fn merge(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    match trait_id {
      1 => { 
        if self.skin { Err(SlaErrors::MergeCheckFailed) } 
        else { 
          self.skin = true;
          Ok(()) 
        }
      },
      2 => {
        if self.clothing { Err(SlaErrors::MergeCheckFailed) }
        else { 
          self.clothing = true;
          Ok(()) 
        }
      },
      3 => {
        if self.eyes { Err(SlaErrors::MergeCheckFailed) }
        else { 
          self.eyes = true;
          Ok(()) 
        }
      },
      4 => {
        if self.hat { Err(SlaErrors::MergeCheckFailed) }
        else { 
          self.hat = true;
          Ok(()) 
        }
      },
      5 => {
        if self.mouth { Err(SlaErrors::MergeCheckFailed) }
        else { 
          self.mouth = true;
          Ok(()) 
        }
      },
      _ => Err(SlaErrors::TraitTypeInvalid),
    }
  }
}

#[account]
#[derive(Copy, Default)]
pub struct Ranking {
  pub ranking: Option<FungibleAsset>,
  pub minted_next: bool,
}

impl Ranking {
  pub const LEN: usize = DISCRIMINATOR_LENGTH + 2 * 8;

  pub fn check_upgrade_is_allowed(&self, asset: FungibleAsset) -> Result<(), SlaErrors> {
    
    let mint_allowed = match asset {
      FungibleAsset::BADGE_BRONZE => self.ranking.is_none(),
      FungibleAsset::BADGE_SILVER => self.ranking == Some(FungibleAsset::BADGE_BRONZE),
      FungibleAsset::BADGE_GOLD => self.ranking == Some(FungibleAsset::BADGE_SILVER),
      FungibleAsset::BADGE_PLATINUM => self.ranking == Some(FungibleAsset::BADGE_GOLD),
      FungibleAsset::BADGE_DIAMOND => self.ranking == Some(FungibleAsset::BADGE_PLATINUM),
      _ => false,
    };

    if !mint_allowed {
      Err(SlaErrors::NotAllowedToMintBadge)
    } else {
      Ok(())
    }
  }

  pub fn mint_next(&mut self) -> Result<(), SlaErrors> {
    if self.minted_next {
      Err(SlaErrors::NextBadgeAlreadyMinted)
    } else {
      self.minted_next = true;
      Ok(())
    }
  }

  pub fn update_ranking(&mut self, asset: FungibleAsset) -> Result<(), SlaErrors> {
    self.check_upgrade_is_allowed(asset)?;
    self.ranking = Some(asset);
    self.minted_next = false;
    Ok(())
  }
}


#[account]
#[derive(Default)]
pub struct BadgeSupplyCounter {
  pub bronze: u16,
  pub silver: u16,
  pub gold: u16,
  pub platinum: u16,
  pub diamond: u16,
}

impl BadgeSupplyCounter {
  pub const LEN: usize = DISCRIMINATOR_LENGTH + 20;

  pub fn init(&mut self, n_bronze: u16, n_silver: u16, n_gold: u16, n_platinum: u16, n_diamond: u16) {
    self.bronze = n_bronze;
    self.silver = n_silver;
    self.gold = n_gold;
    self.platinum = n_platinum;
    self.diamond = n_diamond;

    msg!("Total supply of badges: {}", self);
  }

  fn check_supply(&self, asset: FungibleAsset) -> Result<(), SlaErrors> {
    let ok = match asset {
      FungibleAsset::BADGE_BRONZE => self.bronze < sla_constants::SUPPLY_BADGE_BRONZE,
      FungibleAsset::BADGE_SILVER => self.silver < sla_constants::SUPPLY_BADGE_SILVER,
      FungibleAsset::BADGE_GOLD => self.gold < sla_constants::SUPPLY_BADGE_GOLD,
      FungibleAsset::BADGE_PLATINUM => self.platinum < sla_constants::SUPPLY_BADGE_PLATINUM,
      FungibleAsset::BADGE_DIAMOND => self.diamond < sla_constants::SUPPLY_BADGE_DIAMOND,
      _ => false,
    };

    if ok {
      Ok(())
    } else {
      Err(SlaErrors::AssetMaxSupplyReached)
    }
  }

  pub fn increment(&mut self, asset: FungibleAsset) -> Result<(), SlaErrors> {
    msg!("Incrementing supply of {} (currently: {})", asset, self);
    self.check_supply(asset)?;
    match asset {
      FungibleAsset::BADGE_BRONZE => self.bronze += 1,
      FungibleAsset::BADGE_SILVER => self.silver += 1,
      FungibleAsset::BADGE_GOLD => self.gold += 1,
      FungibleAsset::BADGE_PLATINUM => self.platinum += 1,
      FungibleAsset::BADGE_DIAMOND => self.diamond += 1,
      _ => panic!("Cannot increment supply counter: asset is not a badge"),
    }
    Ok(())
  }
}

impl fmt::Display for BadgeSupplyCounter {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, 
        "Bronze: {}, silver: {}, gold: {}, platinum: {}, diamond: {}", 
        self.bronze, self.silver, self.gold, self.platinum, self.diamond
      )
  }
}