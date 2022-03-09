use anchor_lang::prelude::*;
use anchor_spl;

use crate::sla_constants::{PREFIX_TREASURY, PREFIX_MASTER};


pub fn mint_edition_unique_token<'info> (
  mint: AccountInfo<'info>,
  ata: AccountInfo<'info>,
  ata_account_owner: AccountInfo<'info>,
  mint_authority: AccountInfo<'info>,
  final_ata_account_owner: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  treasury_bump: u8,
) -> ProgramResult {

  let signer_seeds = &[&[PREFIX_TREASURY.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];

  // Mint master edition to ATA owned by the Creator
  mint_tokens(mint, ata.clone(), mint_authority.clone(), token_program.clone(), Some(signer_seeds), 1)?;

  // Tranfer ownership of the ATA if needed
  if ata_account_owner.key() != final_ata_account_owner.key() {

    let accounts = anchor_spl::token::SetAuthority {
      account_or_mint: ata.clone(),
      current_authority: ata_account_owner,
    };
    let cpi_ctx = CpiContext::new_with_signer(
      token_program,
      accounts,
      signer_seeds,
    );
    anchor_spl::token::set_authority(
      cpi_ctx,
      spl_token::instruction::AuthorityType::AccountOwner,
      Some(final_ata_account_owner.key()),
    )?;
  }

  Ok(())
}

pub fn mint_tokens<'info>(
  mint: AccountInfo<'info>,
  ata: AccountInfo<'info>,
  mint_authority: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  signer_seeds: Option<&[&[&[u8]]]>,
  amount: u64,
) -> ProgramResult {

  let accounts = anchor_spl::token::MintTo {
    mint: mint,
    to: ata, 
    authority: mint_authority,
  };
  
  let cpi_ctx = match signer_seeds {
    Some(seeds) => CpiContext::new_with_signer(token_program, accounts, seeds),
    None => CpiContext::new(token_program, accounts),
  };

  anchor_spl::token::mint_to(cpi_ctx, amount)
}


/// Mint a Hay token to a user's wallet. 
/// The SLA Treasury PDA signs and pays the transaction fees. 
pub fn deprecated_mint_hay<'info> (
  mint: AccountInfo<'info>,
  to: AccountInfo<'info>,
  treasury: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  treasury_bump: u8,
) -> ProgramResult {

  let signer_seeds = &[&[PREFIX_TREASURY.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];

  let cpi_accounts = anchor_spl::token::MintTo {
    mint: mint, 
    to: to,
    authority: treasury,
  };

  let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

  anchor_spl::token::mint_to(cpi_ctx, 1)
}


pub fn burn_tokens<'info>(
  mint: AccountInfo<'info>,
  ata: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  signer_seeds: Option<&[&[&[u8]]]>,
  amount: u64,
) -> ProgramResult {

  let accounts = anchor_spl::token::Burn {
    mint: mint,
    to: ata,
    authority: authority,
  };

  let cpi_ctx = match signer_seeds {
    Some(seeds) => CpiContext::new_with_signer(token_program, accounts, seeds),
    None => CpiContext::new(token_program, accounts),
  };

  anchor_spl::token::burn(cpi_ctx, amount)
}


pub fn mint_whitelist_token<'info>(
  mint: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  master_bump: u8,
) -> ProgramResult {

  let signer_seeds = &[&[PREFIX_MASTER.as_bytes(), bytemuck::bytes_of(&master_bump)][..]];

  // Mint a new Whitelist Token
  let cpi_accounts = anchor_spl::token::MintTo {
    mint: mint,
    to: to.clone(),
    authority: authority.clone()
  };
  let cpi_ctx = CpiContext::new_with_signer(token_program.clone(), cpi_accounts, signer_seeds);
  anchor_spl::token::mint_to(cpi_ctx, 1)
}


pub fn burn_trait<'info>(
  token_account: AccountInfo<'info>, 
  mint_account: AccountInfo<'info>, 
  owner_account: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
) -> ProgramResult {

  let cpi_accounts = anchor_spl::token::Burn {
    mint: mint_account,
    to: token_account,
    authority: owner_account,
  };
  let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
  anchor_spl::token::burn(cpi_ctx, 1)
}
