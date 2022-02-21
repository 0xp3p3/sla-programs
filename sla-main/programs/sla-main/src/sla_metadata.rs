use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;
use mpl_token_metadata::{instruction::update_metadata_accounts_v2, ID, state};

use crate::SLA_PDA_SEED;

pub fn update_metadata<'info>(
  metadata_account: AccountInfo<'info>, 
  update_authority: AccountInfo<'info>, 
  metadata_program: AccountInfo<'info>,
  master_bump: u8,
  new_uri: String
) -> ProgramResult {

  let metadata = state::Metadata::from_account_info(&metadata_account)?;

  // Update the URI field in the data
  let data = state::DataV2 {
    name: metadata.data.name,
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

  // Send and sign the transaction using the update authority PDA
  invoke_signed(
    &instruction,
    accounts,
    &[&[SLA_PDA_SEED.as_bytes(), &[master_bump]]],
  )
}
