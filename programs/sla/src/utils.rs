use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_spl;
use mpl_token_metadata;

use crate::sla_collection::check_collection;
use crate::sla_metadata::check_verified_creator;
use crate::sla_constants::{LLAMA_COLLECTION, COLLECTIONS_CREATOR_WALLET};
use crate::SlaErrors; 


pub fn transfer<'info>(from: AccountInfo<'info>, to: AccountInfo<'info>, lambports: u64) -> Result<(), ProgramError> {

  let instruction = solana_program::system_instruction::transfer(&from.key(), &to.key(), lambports);

  solana_program::program::invoke(
    &instruction,
    &[from, to]
  )
}

pub fn str_to_pubkey(key: &str) -> Pubkey {
  Pubkey::from_str(key).unwrap()
}

pub fn assert_address<'info>(
  given: &Pubkey,
  expected: &str,
) -> bool {
  given == &str_to_pubkey(expected)
}

pub fn verify_avatar<'info>(
  mint: Pubkey,
  ata: Account<'info, anchor_spl::token::TokenAccount>,
  user: Pubkey,
  avatar_metadata: &AccountInfo<'info>,
) -> Result<(), SlaErrors> {

  // Check the ATA account contains exactly 1 token
  if !(ata.amount == 1) {
    return Err(SlaErrors::AtaAmountIsNotOne)
  }

  // Check the mint of the ATA is the right one
  if !(ata.mint == mint) {
    return Err(SlaErrors::MintAndAtaMismatch)
  }

  // Check the user is the owner of the ATA
  if !(ata.owner == user) {
    return Err(SlaErrors::TokenPDAMismatch)
  }

  // Check that we are in the list of creators and are verified
  let metadata = mpl_token_metadata::state::Metadata::from_account_info(avatar_metadata).unwrap();
  let expected = &str_to_pubkey(COLLECTIONS_CREATOR_WALLET);
  if !check_verified_creator(metadata.clone(), expected) {
    return Err(SlaErrors::CreatorInvalid)
  }

  // Check the collection is from the Avatar collection
  let expected = &str_to_pubkey(LLAMA_COLLECTION);
  if !check_collection(metadata, expected) {
    return Err(SlaErrors::AvatarNotInCollection)
  }

  Ok(())  
}
