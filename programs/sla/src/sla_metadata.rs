use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};
use mpl_token_metadata::{
  instruction::{update_metadata_accounts_v2, create_metadata_accounts_v2}, 
  ID, state
};

use crate::sla_constants::{PREFIX_TREASURY};


pub fn create_metadata_account<'info>(
  metadata: AccountInfo<'info>,
  new_mint: AccountInfo<'info>,
  treasury: AccountInfo<'info>,
  creator: AccountInfo<'info>,
  community_wallet: Pubkey,
  name: String, 
  symbol: String,
  uri: String,
  seller_fee_basis_points: u16,
  system_program: AccountInfo<'info>,
  rent_sysvar: AccountInfo<'info>,
  mpl_token_metadata_program: AccountInfo<'info>,
  collection: Option<mpl_token_metadata::state::Collection>,
  treasury_bump: u8,
) -> ProgramResult {

  let creator_wallet = state::Creator {
    address: creator.key(),
    verified: true,
    share: 30,
  };

  let community_wallet = state::Creator {
    address: community_wallet,
    verified: false,
    share: 70,
  };

  let instruction = create_metadata_accounts_v2(
    ID, 
    metadata.key(), 
    new_mint.key(), 
    treasury.key(),  // mint authority
    creator.key(),  // payer
    creator.key(),  // update authority
    name, symbol, uri, 
    Some(vec![creator_wallet, community_wallet]), 
    seller_fee_basis_points, 
    true,   // update_authority_is_signer
    false,  // is_mutable 
    collection,  // collection 
    None,  // uses  
  );

  let accounts = &[
    metadata,
    new_mint,
    treasury.clone(),  // mint authority
    creator.clone(),  // payer
    creator,  // update authority
    system_program,
    rent_sysvar, 
    mpl_token_metadata_program
  ];

  invoke_signed(
    &instruction, accounts, 
    &[&[PREFIX_TREASURY.as_bytes(), &[treasury_bump]]]
  )
}

pub fn set_primary_sale_happened<'info>(
  metadata: AccountInfo<'info>,
  update_authority: AccountInfo<'info>,
  treasury_bump: u8,
) -> ProgramResult {

  let instruction = mpl_token_metadata::instruction::update_metadata_accounts_v2(
    mpl_token_metadata::ID,
    metadata.key(),
    update_authority.key(),
    None,  // new update authority
    None,  // new data
    Some(true),  // primary_sale_happened
    None,  // is_mutable
  );

  let accounts = &[
    metadata,
    update_authority,
  ];

  invoke_signed(
    &instruction,
    accounts,
    &[&[PREFIX_TREASURY.as_bytes(), &[treasury_bump]]],
  )
}


pub fn update_metadata<'info>(
  metadata_account: AccountInfo<'info>, 
  update_authority: AccountInfo<'info>, 
  metadata_program: AccountInfo<'info>,
  new_uri: String,
  new_name: Option<String>,
) -> ProgramResult {

  let metadata = state::Metadata::from_account_info(&metadata_account)?;

  // Update the URI field in the data
  let data = state::DataV2 {
    name: match new_name {
      Some(name) => name,
      None => metadata.data.name,
    },
    symbol: metadata.data.symbol,
    uri: new_uri,
    seller_fee_basis_points: metadata.data.seller_fee_basis_points,
    creators: metadata.data.creators,
    collection: metadata.collection,
    uses: metadata.uses
  };

  // Create the Metaplex Metadata instruction to update the metadata of the Avatar
  let instruction = update_metadata_accounts_v2(
    ID,
    metadata_account.key(),
    update_authority.key(),
    None,  // new update_authority
    Some(data),  // new Data
    None,  // primary_sale_happened
    None,  // is_mutable
  );

  let accounts = &[metadata_program, metadata_account, update_authority];

  // Send the transaction
  invoke(&instruction, accounts)
}

pub fn check_verified_creator(
  metaadata: mpl_token_metadata::state::Metadata,
  expected: &Pubkey,
) -> bool {
  match &metaadata.data.creators {
    Some(creators) => {
      creators.iter().any(|c| c.verified && c.address == *expected)
    },
    None => false,
  }
}
