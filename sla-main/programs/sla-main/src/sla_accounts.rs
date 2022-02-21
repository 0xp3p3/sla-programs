use anchor_lang::prelude::*;
use crate::SlaErrors;

const DISCRIMINATOR_LENGTH: usize = 8;

#[account]
#[derive(Copy, Default)]
pub struct AvatarAccount {
  pub traits: Option<AvatarData>,
}

impl AvatarAccount {
  pub const LEN: usize = DISCRIMINATOR_LENGTH + AvatarData::LEN + 1;

  pub fn init(&mut self) -> Result<(), SlaErrors> {
    match self.traits {
      None => {
        self.traits = Some(AvatarData::init());
        Ok(())
      }
      Some(_) => Err(SlaErrors::AvatarAlreadyInitialized),
    }
  }

  pub fn mint_trait(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    match &mut self.traits {
      Some(traits) => {
        if traits.mint_if_allowed(trait_id) {
          Ok(())
        } else {
          Err(SlaErrors::AvatarCannotMintTrait)
        }
      },
      None => Err(SlaErrors::AvatarPDANotInitialized),
    }
  }

  pub fn check_merge_is_allowed(&self, trait_id: u8) -> Result<(), SlaErrors> {
    match &self.traits {
      Some(traits) => {
        if traits.check_merge_is_allowed(trait_id) {
          Ok(())
        } else {
          Err(SlaErrors::MergeCheckFailed)
        }
      },
      None => Err(SlaErrors::MergeCheckFailedBecauseAvatarNotInitialized),
    }
  }

  pub fn merge(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    self.check_merge_is_allowed(trait_id)?;

    match &mut self.traits {
      Some(traits) => traits.merge(trait_id),
      None => Err(SlaErrors::AvatarPDANotInitialized)
    }
  }
}

#[derive(Clone, Copy, Default, AnchorSerialize, AnchorDeserialize)]
pub struct AvatarData {
  pub skin: AvatarTraitData,
  pub clothing: AvatarTraitData,
  pub eyes: AvatarTraitData,
  pub hat: AvatarTraitData,
  pub mouth: AvatarTraitData,
}

impl AvatarData {
  const LEN: usize = 5 * AvatarTraitData::LEN;

  pub fn init() -> Self {
    AvatarData {
      skin: AvatarTraitData::new(false),
      clothing: AvatarTraitData::new(false),
      eyes: AvatarTraitData::new(false),
      hat: AvatarTraitData::new(false),
      mouth: AvatarTraitData::new(false),
    }
  }

  fn mint_if_allowed(&mut self, trait_id: u8) -> bool {
    match trait_id {
      2 => self.skin.mint(),
      3 => self.clothing.mint(),
      4 => self.eyes.mint(),
      5 => self.hat.mint(),
      6 => self.mouth.mint(),
      _ => false
    }
  }

  fn check_merge_is_allowed(&self, trait_id: u8) -> bool {
    match trait_id {
      2 => self.skin.is_merge_allowed(),
      3 => self.clothing.is_merge_allowed(),
      4 => self.eyes.is_merge_allowed(),
      5 => self.hat.is_merge_allowed(),
      6 => self.mouth.is_merge_allowed(),
      _ => false
    }
  }

  fn merge(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    match trait_id {
      2 => { 
        self.skin.merge();
        Ok(())
      },
      3 => {
        self.clothing.merge();
        Ok(())
      },
      4 => {
        self.eyes.merge();
        Ok(())
      },
      5 => {
        self.hat.merge();
        Ok(())
      },
      6 => {
        self.mouth.merge();
        Ok(())
      },
      _ => Err(SlaErrors::TraitTypeInvalid),
    }
  }
}

#[derive(Clone, Copy, Default, AnchorSerialize, AnchorDeserialize)]
pub struct AvatarTraitData {
  pub minted: bool,
  pub merged: bool,
}

impl AvatarTraitData {
  const LEN: usize = 2;

  fn new(already_present: bool) -> Self {
    Self {
      minted: already_present,
      merged: already_present,
    }
  }

  fn mint(&mut self) -> bool {
    if self.minted {
      false 
    } else {
      self.minted = true;
      true
    }
  }

  fn is_merge_allowed(&self) -> bool {
    !self.merged
  }
  
  fn merge(&mut self) {
    self.merged = true;
  }
}
