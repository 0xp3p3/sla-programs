use anchor_lang::prelude::*;

use crate::SlaErrors;

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
