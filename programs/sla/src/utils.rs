use anchor_lang::prelude::*;
use anchor_spl;
use mpl_token_metadata;
use std::str::FromStr;

use crate::sla_collection::{check_collection, extract_trait_id};
use crate::sla_constants;
use crate::SlaErrors;


pub fn str_to_pubkey(key: &str) -> Pubkey {
  Pubkey::from_str(key).unwrap()
}

pub fn assert_address<'info>(given: &Pubkey, expected: &str) -> bool {
  given == &str_to_pubkey(expected)
}

fn verify_nft<'info>(
  mint: Pubkey,
  ata: Account<'info, anchor_spl::token::TokenAccount>,
  user: Pubkey,
  avatar_metadata: &AccountInfo<'info>,
  expected_collection: &Pubkey,
) -> Result<(), SlaErrors> {
  // Check the ATA account contains exactly 1 token
  if !(ata.amount == 1) {
    return Err(SlaErrors::AtaAmountIsNotOne);
  }

  // Check the mint of the ATA is the right one
  if !(ata.mint == mint) {
    return Err(SlaErrors::MintAndAtaMismatch);
  }

  // Check the user is the owner of the ATA
  if !(ata.owner == user) {
    return Err(SlaErrors::TokenPDAMismatch);
  }

  // Check that we are in the list of creators and are verified
  let metadata = mpl_token_metadata::state::Metadata::from_account_info(avatar_metadata).unwrap();

  // Check the collection is from the expected collection and that it is verified
  if !check_collection(metadata, expected_collection) {
    return Err(SlaErrors::AvatarNotInCollection);
  }

  Ok(())
}

pub fn verify_avatar<'info>(
  mint: Pubkey,
  ata: Account<'info, anchor_spl::token::TokenAccount>,
  user: Pubkey,
  avatar_metadata: &AccountInfo<'info>,
) -> Result<(), SlaErrors> {
  verify_nft(
    mint,
    ata,
    user,
    avatar_metadata,
    &str_to_pubkey(sla_constants::LLAMA_COLLECTION),
  )
}

pub fn verify_trait<'info>(
  mint: Pubkey,
  ata: Account<'info, anchor_spl::token::TokenAccount>,
  user: Pubkey,
  trait_metadata: &AccountInfo<'info>,
) -> Result<u8, SlaErrors> {
  // Fetch metadata account
  let metadata = mpl_token_metadata::state::Metadata::from_account_info(trait_metadata).unwrap();

  // Check which collection the trait is part of
  let (collection, trait_id) = extract_trait_id(metadata)?;

  // Verify the NFT
  verify_nft(mint, ata, user, trait_metadata, &collection)?;

  Ok(trait_id)
}
