use anchor_lang::prelude::*;
use mpl_token_metadata;
use solana_program::program::{invoke_signed};

use crate::sla_constants::PREFIX_TREASURY;

pub fn verify_collection<'info>(
  metadata_to_verify: AccountInfo<'info>,
  creator: AccountInfo<'info>,
  collection_mint: AccountInfo<'info>,
  collection_metadata: AccountInfo<'info>,
  collection_master_edition: AccountInfo<'info>,
  treasury_bump: u8,
) -> ProgramResult {

  // Verify the Collection
  msg!("Verifying the collection");
  let instruction = mpl_token_metadata::instruction::verify_collection(
    mpl_token_metadata::ID,
    metadata_to_verify.key(),
    creator.key(),  // collection update authority
    creator.key(),  // payer
    collection_mint.key(),  // collection mint
    collection_metadata.key(),  // collection metadata
    collection_master_edition.key(),  // collection master edition
    None,  // collection authority record
  );

  let accounts = &[
    metadata_to_verify,
    creator.clone(),  // collection update authority
    creator, // payer
    collection_mint,  // collection mint
    collection_metadata,  // collection Metadata
    collection_master_edition,  // collection Master Edition
  ];

  invoke_signed(
    &instruction, 
    accounts,
    &[&[PREFIX_TREASURY.as_bytes(), &[treasury_bump]]]
  )?;

  Ok(())
}

pub fn check_collection(
  collection_member: mpl_token_metadata::state::Metadata,
  expected: &Pubkey,
) -> bool {
  match &collection_member.collection {
    Some(collection) => {
      collection.key == *expected && collection.verified
    },
    None => false,
  }
}
