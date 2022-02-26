use anchor_lang::{prelude::*};
use anchor_spl::{token, associated_token};
use mpl_token_metadata;

mod sla_accounts;
mod sla_errors;
mod utils;
mod sla_metadata;
mod sla_token;
mod sla_whitelist;
mod sla_hay;

use sla_errors::SlaErrors;

declare_id!("2sochV4HLApPhUPHTYZstZDVdfkxQC1MUyrTVU6TSAxj");

// const ARWEAVE_WALLET: &[u8] = b"JDpq9RP9zUdVShvwwp2DK8orxU8e73SDMsQiYnsK87ga";
const SLA_MASTER_SEED: &str = "sla_master";
const SLA_LLAMA_SEED: &str = "sla_llama";
const SLA_TREASURY_SEED: &str = "sla_treasury";

#[program]
pub mod sla_main {
    use super::*;

    pub fn mint_hay(
      ctx: Context<MintHay>,
      treasury_bump: u8,
      llama_bump: u8,
    ) -> ProgramResult {

      let llama = &mut ctx.accounts.llama;

      // Check if this Llama is allowed to mint Hay at this time of day,
      llama.mint_hay(ctx.accounts.clock.unix_timestamp)?;

      // Mint some Hay
      sla_token::mint_hay(
        ctx.accounts.hay_mint.to_account_info(),
        ctx.accounts.hay_token.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        treasury_bump
      )?;

      Ok(())
    }

    // pub fn mint_trait_whitelist_token(
    //   ctx: Context<MintTraitWhitelistToken>, 
    //   avatar_bump: u8, 
    //   master_bump: u8,
    //   trait_id: u8
    // ) -> ProgramResult {
      
    //   // Initialize the Avatar account if needed
    //   let avatar = &mut ctx.accounts.avatar;
    //   match avatar.traits {
    //     Some(_) => (),
    //     None => avatar.init()?
    //   };

    //   // Update the Avatar account and check that the user is indeed allowed
    //   // to mint this trait
    //   avatar.mint_trait(trait_id)?;

    //   // Give the user a Whitelist Token
    //   sla_token::mint_whitelist_token(
    //     ctx.accounts.whitelist_mint.to_account_info(),
    //     ctx.accounts.whitelist_token.to_account_info(),
    //     ctx.accounts.sla_master_pda.to_account_info(),
    //     ctx.accounts.token_program.to_account_info(),
    //     master_bump
    //   )?;

    //   Ok(())
    // }

    // Check whether a given Trait NFT can be merged with the specified Avatar NFT
    pub fn check_merge_is_allowed(
      ctx: Context<CheckMergeIsAllowed>, 
      avatar_bump: u8,
      new_trait_id: u8
    ) -> ProgramResult {
    
      // Initialize the Avatar account if needed
      let avatar = &mut ctx.accounts.avatar;
      match avatar.traits {
        Some(_) => (),
        None => avatar.init()?
      };
    
      // Check if the Avatar can still merge this trait
      avatar.check_merge_is_allowed(new_trait_id)?;
      Ok(())
    }

    pub fn merge(
      ctx: Context<Merge>,
      master_bump: u8,
      avatar_bump: u8,
      new_trait_id: u8,
      metadata_uri: String,
      upload_cost: u64,
    ) -> ProgramResult {

      let avatar = &mut ctx.accounts.avatar;
      let metadata_account = ctx.accounts.avatar_metadata.to_account_info();
      let sla_master_pda = ctx.accounts.sla_master_pda.to_account_info();
      let metadata_program = ctx.accounts.metadata_program.to_account_info();
      let payer = ctx.accounts.payer.to_account_info();

      // Update the SLA Avatar data (while checking whether the merge is allowed)
      avatar.merge(new_trait_id)?;

      // Update the metadata URI through the Metaplex program
      sla_metadata::update_metadata(
        metadata_account, 
        sla_master_pda, 
        metadata_program,
        master_bump,
        metadata_uri,
      )?;

      // Burn the trait token
      sla_token::burn_trait(
        ctx.accounts.trait_token.to_account_info(), 
        ctx.accounts.trait_mint.to_account_info(), 
        payer.clone().to_account_info(), 
        ctx.accounts.token_program.to_account_info()
      )?;

      // Reimburse the wallet Arweave uploader
      utils::transfer(
        payer.clone().to_account_info(),
        ctx.accounts.arweave_wallet.to_account_info(), 
        upload_cost
      )?;

      Ok(())
    }
}

#[derive(Accounts)]
#[instruction(treasury_bump: u8, llama_bump: u8)]
pub struct MintHay<'info> {
  #[account(mut)]
  pub hay_mint: Account<'info, token::Mint>,
  pub llama_mint: Account<'info, token::Mint>,

  #[account(
    init_if_needed,
    payer = fee_payer,
    associated_token::mint = hay_mint,
    associated_token::authority = user,
  )]
  pub hay_token: Account<'info, token::TokenAccount>,

  #[account(
    associated_token::mint = llama_mint,
    associated_token::authority = user,
  )]
  pub llama_token: Account<'info, token::TokenAccount>,

  // This is the PDA associated with the Llama NFT
  #[account(
    init_if_needed,
    seeds = [SLA_LLAMA_SEED.as_bytes(), &llama_mint.key().to_bytes()],
    bump = llama_bump,
    payer = fee_payer,
    space = sla_accounts::AvatarAccount::LEN,
  )]
  pub llama: Account<'info, sla_accounts::AvatarAccount>,

  // This is the person owning the Llama NFT to whom we are airdropping 
  // a Hay token
  pub user: AccountInfo<'info>,

  // This is the SLA Treasury PDA
  #[account(
    init_if_needed,
    seeds = [SLA_TREASURY_SEED.as_bytes()],
    bump = treasury_bump,
    space = 8,
    payer = fee_payer,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(mut)]
  pub fee_payer: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub clock: Sysvar<'info, Clock>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  pub system_program: Program<'info, System>,
}


// #[derive(Accounts)]
// #[instruction(avatar_bump: u8, master_bump: u8, trait_id: u8)]
// pub struct MintTraitWhitelistToken<'info> {
//   #[account(
//     init_if_needed,
//     seeds = [SLA_LLAMA_SEED.as_bytes(), &avatar_mint.key().to_bytes()],
//     bump = avatar_bump,
//     payer = payer,
//     space = sla_accounts::AvatarAccount::LEN,
//   )]
//   pub avatar: Account<'info, sla_accounts::AvatarAccount>,

//   pub avatar_mint: Account<'info, token::Mint>,

//   #[account(
//     associated_token::mint = avatar_mint,
//     associated_token::authority = payer,
//   )]
//   pub avatar_token: Account<'info, token::TokenAccount>,

//   #[account(
//     init_if_needed,
//     seeds = [SLA_LLAMA_SEED.as_bytes()],
//     bump = master_bump,
//     payer = payer,
//     space = 8,
//   )]
//   pub sla_master_pda: AccountInfo<'info>,

//   #[account(
//     init_if_needed,
//     payer = payer,
//     associated_token::mint = whitelist_mint,
//     associated_token::authority = payer,
//   )]
//   pub whitelist_token: Account<'info, token::TokenAccount>,

//   #[account(
//     mut,
//     constraint = sla_whitelist::check_whitelist_mint_id(&whitelist_mint.key(), trait_id) 
//       @ SlaErrors::InvalidWhitelistMint,
//   )]
//   pub whitelist_mint: Account<'info, token::Mint>,

//   #[account(mut)]
//   pub payer: Signer<'info>,

//   pub token_program: Program<'info, token::Token>,
//   pub system_program: Program<'info, System>,
//   pub rent: Sysvar<'info, Rent>,
//   pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
// }


#[derive(Accounts)]
#[instruction(avatar_bump: u8)]
pub struct CheckMergeIsAllowed<'info> {
  #[account(
    init_if_needed,
    seeds = [SLA_LLAMA_SEED.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = avatar_bump,
    payer = payer, 
    space = sla_accounts::AvatarAccount::LEN,
  )]
  pub avatar: Account<'info, sla_accounts::AvatarAccount>,
  
  pub avatar_mint: Account<'info, token::Mint>,
  pub trait_mint: Account<'info, token::Mint>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = payer,
  )]
  pub avatar_token: Account<'info, token::TokenAccount>,

  #[account(
    associated_token::mint = trait_mint,
    associated_token::authority = payer,
  )]
  pub trait_token: Account<'info, token::TokenAccount>,
  
  #[account(mut)]
  pub payer: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(master_bump: u8, avatar_bump: u8)]
pub struct Merge<'info> {
  #[account(
    mut,
    seeds = [SLA_LLAMA_SEED.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = avatar_bump,
  )]
  pub avatar: Account<'info, sla_accounts::AvatarAccount>,
  
  pub avatar_mint: Account<'info, token::Mint>,

  #[account(mut)]
  pub trait_mint: Account<'info, token::Mint>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = payer,
  )]
  pub avatar_token: Account<'info, token::TokenAccount>,

  #[account(
    mut,
    associated_token::mint = trait_mint,
    associated_token::authority = payer,
  )]
  pub trait_token: Account<'info, token::TokenAccount>,
  
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(mut)]
  pub avatar_metadata: AccountInfo<'info>,

  #[account(
    seeds = [SLA_MASTER_SEED.as_bytes()],
    bump = master_bump,
  )]
  pub sla_master_pda: AccountInfo<'info>,

  // The wallet paying for Arweave upload transactions
  #[account(mut)]
  pub arweave_wallet: AccountInfo<'info>,

  #[account(address = token::ID)]
  pub token_program: AccountInfo<'info>,

  #[account(address = mpl_token_metadata::ID)]
  pub metadata_program: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}
