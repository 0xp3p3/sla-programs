use anchor_lang::prelude::*;
use crate::SlaErrors;
use crate::input::AvatarInitTraits;

const DISCRIMINATOR_LENGTH: usize = 8;

#[account]
#[derive(Copy, Default)]
pub struct AvatarAccount {
  pub traits: Option<AvatarData>,
}

impl AvatarAccount {
  pub const LEN: usize = DISCRIMINATOR_LENGTH + AvatarData::LEN + 1;

  pub fn init(&mut self, traits: AvatarInitTraits) -> Result<(), SlaErrors> {
    match self.traits {
      None => {
        self.traits = Some(AvatarData::init(traits));
        Ok(())
      }
      Some(_) => Err(SlaErrors::AvatarAlreadyInitialized),
    }
  }

  pub fn check_merge_is_allowed(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    match &mut self.traits {
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
}

#[account]
#[derive(Copy, Default)]
pub struct TraitAccount {
  pub trait_type: Option<TraitType>,
}

impl TraitAccount {
  pub const LEN: usize = DISCRIMINATOR_LENGTH + TraitType::LEN;

  pub fn init(&mut self, trait_id: u8) -> Result<(), SlaErrors> {
    match self.trait_type {
      None => {
        self.trait_type = Some(TraitType::get_trait(trait_id)?);
        Ok(())
      }
      Some(_) => Err(SlaErrors::TraitAlreadyInitialized),
    }
  }
}

#[derive(Clone, Copy, Default, AnchorSerialize, AnchorDeserialize)]
pub struct AvatarData {
  pub background: AvatarTraitData,
  pub skin: AvatarTraitData,
  pub clothes: AvatarTraitData,
  pub eyes: AvatarTraitData,
  pub hat: AvatarTraitData,
  pub mouth: AvatarTraitData,
}

impl AvatarData {
  const LEN: usize = 3 * AvatarTraitData::LEN;

  pub fn init(traits: AvatarInitTraits) -> Self {
    AvatarData {
      background: AvatarTraitData::new(traits.background),
      skin: AvatarTraitData::new(traits.skin),
      clothes: AvatarTraitData::new(traits.clothes),
      eyes: AvatarTraitData::new(traits.eyes),
      hat: AvatarTraitData::new(traits.hat),
      mouth: AvatarTraitData::new(traits.mouth),
    }
  }

  fn check_merge_is_allowed(&mut self, trait_id: u8) -> bool {
    match trait_id {
      1 => self.background.is_merge_allowed(),
      2 => self.skin.is_merge_allowed(),
      3 => self.clothes.is_merge_allowed(),
      4 => self.eyes.is_merge_allowed(),
      5 => self.hat.is_merge_allowed(),
      6 => self.mouth.is_merge_allowed(),
      _ => false
    }
  }
}

#[derive(Clone, Copy, Default, AnchorSerialize, AnchorDeserialize)]
pub struct AvatarTraitData {
  pub minted: bool,
  pub merged: bool,
  pub merge_checked: bool,
}

impl AvatarTraitData {
  const LEN: usize = 3;

  fn new(already_present: bool) -> Self {
    Self {
      minted: already_present,
      merged: already_present,
      merge_checked: false,
    }
  }

  fn is_merge_allowed(&mut self) -> bool {
    if !self.merged && !self.merge_checked {
      self.merge_checked = true;
      true
    } else {
      false
    }    
  }

  fn is_mint_allowed(self) -> bool {
    !self.minted
  }
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum TraitType {
  Background,
  Skin,
  Clothes,
  Eyes,
  Hat,
  Mouth,
}

impl TraitType {
  const LEN: usize = 2;

  fn get_trait(trait_id: u8) -> Result<Self, SlaErrors> {
    match trait_id {
      1 => Ok(Self::Background),
      2 => Ok(Self::Skin),
      3 => Ok(Self::Clothes),
      4 => Ok(Self::Eyes),
      5 => Ok(Self::Hat),
      6 => Ok(Self::Mouth),
      _ => Err(SlaErrors::TraitTypeInvalid),
    }
  }
}
