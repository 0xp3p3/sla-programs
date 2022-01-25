use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

mod sla_accounts;
mod sla_errors;
mod utils;
mod input;

use sla_errors::SlaErrors;

declare_id!("EzSxPafKqVCRohTEHYtKf7vg1XRxXSv8hxExPwBbkcC2");

const SLA_PDA_SEED: &str = "sla_main";

#[program]
pub mod sla_main {
    use super::*;

    // Initialize the PDA to hold data about an Avatar
    pub fn initialize_avatar_account(
      ctx: Context<InitializeAvatar>, 
      bump: u8, 
      init_traits: input::AvatarInitTraits,
    ) -> ProgramResult {
      ctx.accounts.pda.init(init_traits)?;
      Ok(())
    }

    // Initialize the PDA to hold data about a Trait
    pub fn initialize_trait_account(
      ctx: Context<InitializeTrait>,
      bump: u8,
      trait_type: u8,
    ) -> ProgramResult {
      ctx.accounts.pda.init(trait_type)?;
      Ok(())
    }

    // Check whether a given Trait NFT can be merged with the specified Avatar NFT
    pub fn check_merge_is_allowed(ctx: Context<CheckMergeIsAllowed>, trait_id: u8) -> ProgramResult {
      let avatar_pda = &mut ctx.accounts.avatar_pda;
      avatar_pda.check_merge_is_allowed(trait_id)?;
      Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeAvatar<'info> {
  // Derive a new PDA for this SLA NFT, paid by the user
  #[account(
    init,
    seeds = [SLA_PDA_SEED.as_bytes(), &mint.key().as_ref()],
    bump = bump,
    payer = user,
    space = sla_accounts::AvatarAccount::LEN,
  )]
  pub pda: Account<'info, sla_accounts::AvatarAccount>,

  pub mint: Account<'info, Mint>,

  // Check that the token belongs to the user
  #[account(
    constraint = utils::check_token_owner(&mint.key(), user.key, &token_pda.to_account_info()) 
      @ SlaErrors::TokenPDAMismatch,
  )]
  pub token_pda: Account<'info, TokenAccount>,

  // The user must sign the transaction and will pay for the new account
  // created
  #[account(mut)]
  pub user: Signer<'info>,

  // The system program is required when creating a new account
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeTrait<'info> {
  // Derive a new PDA for this SLA NFT, paid by the user
  #[account(
    init,
    seeds = [SLA_PDA_SEED.as_bytes(), &mint.key().as_ref()],
    bump = bump,
    payer = user,
    space = sla_accounts::TraitAccount::LEN,
  )]
  pub pda: Account<'info, sla_accounts::TraitAccount>,

  pub mint: Account<'info, Mint>,

  // Check that the token belongs to the user
  #[account(
    constraint = utils::check_token_owner(&mint.key(), user.key, &token_pda.to_account_info()) 
      @ SlaErrors::TokenPDAMismatch,
  )]
  pub token_pda: Account<'info, TokenAccount>,

  // The user must sign the transaction and will pay for the new account
  // created
  #[account(mut)]
  pub user: Signer<'info>,

  // The system program is required when creating a new account
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(avatar_bump: u8, trait_bump: u8)]
pub struct CheckMergeIsAllowed<'info> {
  // Check that Avatar PDA exists and that it belongs to this program
  #[account(
    seeds = [SLA_PDA_SEED.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = avatar_bump,
  )]
  pub avatar_pda: Account<'info, sla_accounts::AvatarAccount>,
  
  // Check that Trait PDA exists and that it belongs to this program
  #[account(
    seeds = [SLA_PDA_SEED.as_bytes(), &trait_mint.key().to_bytes()],
    bump = trait_bump,
  )]
  pub trait_pda: Account<'info, sla_accounts::TraitAccount>,
  
  pub avatar_mint: Account<'info, Mint>,
  pub trait_mint: Account<'info, Mint>,

  // Check that the Avatar token belongs to the user
  #[account(
    constraint = utils::check_token_owner(&avatar_mint.key(), user.key, &avatar_token_pda.to_account_info()) 
      @ SlaErrors::TokenPDAMismatch,
  )]
  pub avatar_token_pda: Account<'info, TokenAccount>,

  // Check that the Trait token belongs to the user
  #[account(
    constraint = utils::check_token_owner(&trait_mint.key(), user.key, &trait_token_pda.to_account_info()) 
      @ SlaErrors::TokenPDAMismatch,
  )]
  pub trait_token_pda: Account<'info, TokenAccount>,
  
  // The user must sign the transaction
  pub user: Signer<'info>,
}
