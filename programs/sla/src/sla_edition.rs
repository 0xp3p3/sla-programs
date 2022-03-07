use anchor_lang::prelude::*;
use mpl_token_metadata::{ID, 
  instruction::{create_master_edition_v3, mint_new_edition_from_master_edition_via_token}
};
use solana_program::program::{invoke_signed};

use crate::sla_constants::PREFIX_TREASURY;


pub fn create_master_edition<'info>(
  mint: AccountInfo<'info>,
  metadata: AccountInfo<'info>,
  master_edition: AccountInfo<'info>,
  creator: AccountInfo<'info>,
  treasury: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  system_program: AccountInfo<'info>,
  rent_sysvar: AccountInfo<'info>,
  mpl_token_metadata_program: AccountInfo<'info>,
  treasury_bump: u8,
  max_supply: Option<u64>
) -> ProgramResult {

  let instruction = create_master_edition_v3(
    ID,
    master_edition.key(),
    mint.key(),
    creator.key(),  // update_authority
    treasury.key(),  // mint_authority
    metadata.key(),
    creator.key(),  // payer
    max_supply
  );
    
  let accounts = &[
    master_edition,
    mint,
    creator.clone(),  // update_authority
    treasury,  // mint authority of the metadata
    creator,  // payer
    metadata,
    token_program,
    system_program,
    rent_sysvar,
    mpl_token_metadata_program
  ];

  invoke_signed(
    &instruction, 
    accounts, 
    &[&[PREFIX_TREASURY.as_bytes(), &[treasury_bump]]]
  )
}

pub fn mint_edition_from_master_edition<'info> (
  new_metadata: AccountInfo<'info>,
  new_edition: AccountInfo<'info>,
  master_edition: AccountInfo<'info>,
  new_mint: AccountInfo<'info>,
  edition_marker: AccountInfo<'info>,
  treasury: AccountInfo<'info>,
  user: AccountInfo<'info>,
  master_ata: AccountInfo<'info>,
  master_metadata: AccountInfo<'info>,
  master_mint: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  system_program: AccountInfo<'info>,
  rent_sysvar: AccountInfo<'info>,
  edition_number: u64,
  treasury_bump: u8,
) -> ProgramResult {

  // Check that the new edition is exactly `current_supply + 1`
  let master_edition_data = mpl_token_metadata::state::MasterEditionV2::from_account_info(&master_edition)?;
  assert_eq!(edition_number, master_edition_data.supply + 1);

  let instruction = mint_new_edition_from_master_edition_via_token(
    mpl_token_metadata::ID,
    new_metadata.key(),
    new_edition.key(),
    master_edition.key(),
    new_mint.key(),
    treasury.key(),  // new mint_authority
    user.key(),  // payer
    treasury.key(),  // owner of token account containing master
    master_ata.key(),  // master edition ATA
    treasury.key(),  // new_metadata update_authority
    master_metadata.key(),
    master_mint.key(),
    edition_number,
  );

  let accounts = &[
    new_metadata.clone(),
    new_edition,
    master_edition,
    new_mint,
    edition_marker,
    treasury.clone(),  // new mint_authority
    user,  // payer
    treasury.clone(),  // owner of token account containing master edition token
    master_ata,
    treasury.clone(),  // update_authority for new metadata
    master_metadata, 
    token_program,
    system_program,
    rent_sysvar,
  ];

  msg!("About to send the transaction");
    
  invoke_signed(
    &instruction,
    accounts,
    &[&[PREFIX_TREASURY.as_bytes(), &[treasury_bump]]],
  )
}