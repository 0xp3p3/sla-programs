use anchor_lang::prelude::*;

#[derive(Clone, Copy, Default, AnchorDeserialize, AnchorSerialize)]
pub struct AvatarInitTraits {
  pub background: bool,
  pub skin: bool,
  pub clothes: bool,
  pub eyes: bool,
  pub hat: bool,
  pub mouth: bool,
}