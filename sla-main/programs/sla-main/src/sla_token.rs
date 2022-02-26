use anchor_lang::prelude::*;
use anchor_spl;

use crate::{SLA_TREASURY_SEED, SLA_MASTER_SEED};


/// Mint a Hay token to a user's wallet. 
/// The SLA Treasury PDA signs and pays the transaction fees. 
pub fn mint_hay<'info> (
  mint: AccountInfo<'info>,
  to: AccountInfo<'info>,
  treasury: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  treasury_bump: u8,
) -> ProgramResult {

  let signer_seeds = &[&[SLA_TREASURY_SEED.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];

  let cpi_accounts = anchor_spl::token::MintTo {
    mint: mint, 
    to: to,
    authority: treasury,
  };

  let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

  anchor_spl::token::mint_to(cpi_ctx, 1)
}


pub fn mint_whitelist_token<'info>(
  mint: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  master_bump: u8,
) -> ProgramResult {

  let signer_seeds = &[&[SLA_MASTER_SEED.as_bytes(), bytemuck::bytes_of(&master_bump)][..]];

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

