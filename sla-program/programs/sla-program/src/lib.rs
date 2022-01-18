use anchor_lang::prelude::*;
use solana_program::program::invoke;
use metaplex_token_metadata as mpl_token_metadata;


declare_id!("DMiJ45zEJgZgdZnzxetnuw9aTXbyvdSHP3pgkpL2uQYS");

const SLA_METADATA_PDA_SEED: &str = "sla_metadata";

#[program]
pub mod sla_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: Vec<u8>, bump: u8) -> ProgramResult {

      let instruction = mpl_token_metadata::instruction::update_metadata_accounts(
        mpl_token_metadata::id(), 
        ctx.accounts.metadata_key.key(), 
        ctx.accounts.treasury.key(), 
        Some(ctx.accounts.pda.key()), 
        None, 
        None
      );
      
      let accounts = [
        ctx.accounts.metadata_program.clone(),
        ctx.accounts.metadata_key.clone(), 
        ctx.accounts.treasury.to_account_info().clone()
      ];
      
      invoke(&instruction, &accounts)
    }
}

#[derive(Accounts)]
#[instruction(seed: Vec<u8>, bump: u8)]
pub struct Initialize<'info> {
  #[account(
      init,
      seeds = [SLA_METADATA_PDA_SEED.as_bytes(), &seed],
      bump = bump,
      payer = treasury,
      space = 8 + 8
  )]
  pub pda: AccountInfo<'info>,
  #[account(mut)]
  pub metadata_key: AccountInfo<'info>,
  #[account(mut)]
  pub treasury: Signer<'info>,
  pub metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

// #[account]
// #[derive(Default)]
// pub struct EmptyAccount {}

// #[error]
// pub enum ErrorCode {
//     #[msg("Something went wrong when invoking the metaplex program")]
//     Initialize,
// }