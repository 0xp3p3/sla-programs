use anchor_lang::{prelude::*};
use anchor_spl;
use mpl_token_metadata;

mod sla_accounts;
mod sla_errors;
mod utils;
mod sla_metadata;
mod sla_token;
mod sla_constants;
mod sla_collection;
mod sla_fungible_token;
use sla_errors::SlaErrors;
use utils::{assert_address, verify_avatar, verify_trait};

declare_id!("GUSxqUfUdqchfErA3DrW1jNVJKGdMpxt71AeDkJJtG5R");


#[program]
pub mod sla {
    use super::*;

    pub fn merge(
      ctx: Context<Merge>,
      avatar_bump: u8,
      metadata_uri: String,
    ) -> ProgramResult {

      let avatar = &mut ctx.accounts.avatar;
      let avatar_metadata = ctx.accounts.avatar_metadata.to_account_info();
      let trait_metadata = ctx.accounts.trait_metadata.to_account_info();
      let metadata_program = ctx.accounts.metadata_program.to_account_info();
      let payer = ctx.accounts.payer.to_account_info();
      let combine_authority = ctx.accounts.combine_authority.to_account_info();

      let trait_ata = ctx.accounts.trait_token.to_account_info();
      let trait_mint = ctx.accounts.trait_mint.to_account_info();

      // Verify that the avatar belongs to the SLA collection
      msg!("Verifying agent belongs to the right collection");
      verify_avatar(
        ctx.accounts.avatar_mint.key(),
        ctx.accounts.avatar_token.clone(),
        payer.key(),
        &avatar_metadata,
      )?;

      // Verify that the trait belongs to the SLA collection + extract the trait ID
      msg!("Verifying trait belongs to the right collection");
      let trait_id = verify_trait(
        trait_mint.key(),
        ctx.accounts.trait_token.clone(),
        payer.key(),
        &trait_metadata,
      )?;

      // Initialize the Avatar account if needed
      msg!("Initializing agent PDA if needed");
      match avatar.traits {
        Some(_) => (),
        None => avatar.init()?,
      };

      // Update the SLA Avatar data (while checking whether the merge is allowed)
      msg!("Updating agent PDA");
      avatar.merge(trait_id)?;

      // Update the metadata URI through the Metaplex program
      msg!("Updating agent metadata with new URI");
      sla_metadata::update_metadata(
        avatar_metadata, 
        combine_authority,
        metadata_program,
        metadata_uri,
        None,
      )?;

      // Burn the trait token
      msg!("Burning trait token");
      sla_token::burn_trait(
        trait_ata, 
        trait_mint, 
        payer.clone(), 
        ctx.accounts.token_program.to_account_info()
      )?;

      msg!("Instruction finished");

      Ok(())
    }


    pub fn mint_id_card(ctx: Context<MintIdCard>, treasury_bump: u8, asset_id: u8) -> ProgramResult {
      msg!("Entering the MintIdCard instruction");

      let fungible_asset = sla_fungible_token::FungibleAsset::from_u8(asset_id);
      
      if fungible_asset != sla_fungible_token::FungibleAsset::ID_CARD {
        panic!("The asset_id provided is not an ID card")
      }

      sla_fungible_token::mint_fungible_asset(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.ata.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
        ctx.accounts.hay_user_ata.to_account_info(),
        ctx.accounts.hay_treasury_ata.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        fungible_asset,
        treasury_bump,
      )
    }

    pub fn mint_badge(
      ctx: Context<MintBadge>, 
      treasury_bump: u8, 
      ranking_bump: u8, 
      badge_supply_counter_bump: u8,
      asset_id: u8,
    ) -> ProgramResult {
      msg!("Entering the MintBadge instruction");

      let user = ctx.accounts.user.to_account_info();
      let asset_to_mint = sla_fungible_token::FungibleAsset::from_u8(asset_id);

      // Verify that the avatar belongs to the SLA collection
      msg!("Verifying agent belongs to the right collection");
      verify_avatar(
        ctx.accounts.avatar_mint.key(),
        ctx.accounts.avatar_token.clone(),
        user.key(),
        &ctx.accounts.avatar_metadata.to_account_info(),
      )?;

      // Check the supply has not reached its max + increment the counter
      let badge_supply_counter = &mut ctx.accounts.badge_supply_counter;
      badge_supply_counter.increment(asset_to_mint)?;

      // Update the avatar ranking (+ check if the avatar is allowed to mint this badge)
      msg!("Checking if the ranking upgrade is allowed");
      let ranking = &mut ctx.accounts.ranking;
      ranking.check_upgrade_is_allowed(asset_to_mint)?;
      
      sla_fungible_token::mint_fungible_asset(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.ata.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
        ctx.accounts.hay_user_ata.to_account_info(),
        ctx.accounts.hay_treasury_ata.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        sla_fungible_token::FungibleAsset::from_u8(asset_id),
        treasury_bump,
      )?;

      ranking.mint_next()?;
      Ok(())
    }

    pub fn change_alias(ctx: Context<ChangeAlias>, metadata_uri: String, new_name: String) -> ProgramResult {

      let avatar_metadata = ctx.accounts.avatar_metadata.to_account_info();
      let metadata_program = ctx.accounts.metadata_program.to_account_info();
      let payer = ctx.accounts.payer.to_account_info();
      let combine_authority = ctx.accounts.combine_authority.to_account_info();

      // Verify that the avatar belongs to the SLA collection
      msg!("Verifying agent belongs to the right collection");
      verify_avatar(
        ctx.accounts.avatar_mint.key(),
        ctx.accounts.avatar_token.clone(),
        payer.key(),
        &avatar_metadata,
      )?;

      // Update the metadata URI through the Metaplex program
      msg!("Updating agent metadata with new URI");
      sla_metadata::update_metadata(
        avatar_metadata, 
        combine_authority,
        metadata_program,
        metadata_uri,
        Some(new_name),
      )?;

      // Burn the trait token
      msg!("Burning ID Card token");
      sla_token::burn_trait(
        ctx.accounts.id_card_ata.to_account_info(), 
        ctx.accounts.id_card_mint.to_account_info(), 
        payer.clone(), 
        ctx.accounts.token_program.to_account_info()
      )?;

      msg!("Instruction finished");

      Ok(())
    }

    pub fn merge_badge(
      ctx: Context<MergeBadge>, 
      ranking_bump: u8, 
      asset_id: u8,
      metadata_uri: String,
    ) -> ProgramResult {

      let avatar_metadata = ctx.accounts.avatar_metadata.to_account_info();
      let metadata_program = ctx.accounts.metadata_program.to_account_info();
      let payer = ctx.accounts.payer.to_account_info();
      let combine_authority = ctx.accounts.combine_authority.to_account_info();

      // Verify that the avatar belongs to the SLA collection
      msg!("Verifying agent belongs to the right collection");
      verify_avatar(
        ctx.accounts.avatar_mint.key(),
        ctx.accounts.avatar_token.clone(),
        payer.key(),
        &avatar_metadata,
      )?;

      // Update the metadata URI through the Metaplex program
      msg!("Updating agent metadata with new URI");
      sla_metadata::update_metadata(
        avatar_metadata, 
        combine_authority,
        metadata_program,
        metadata_uri,
        None,
      )?;

      // Burn the trait token
      msg!("Burning Badge token");
      sla_token::burn_trait(
        ctx.accounts.badge_ata.to_account_info(), 
        ctx.accounts.badge_mint.to_account_info(), 
        payer.clone(), 
        ctx.accounts.token_program.to_account_info()
      )?;

      // Update the Ranking PDA data
      msg!("Updating the Ranking PDA account");
      let ranking = &mut ctx.accounts.ranking;
      ranking.update_ranking(sla_fungible_token::FungibleAsset::from_u8(asset_id))?;

      msg!("Instruction finished");
      Ok(())
    }

    // pub fn init_badge_supply_counter(
    //   ctx: Context<InitBadgeSupplyCounter>, badge_supply_counter_bump: u8,
    //   n_bronze: u16, n_silver: u16, n_gold: u16, n_platinum: u16, n_diamond: u16,
    // ) -> ProgramResult {
    //   ctx.accounts.badge_supply_counter.init(n_bronze, n_silver, n_gold, n_platinum, n_diamond);
    //   Ok(())
    // }
}


#[derive(Accounts)]
#[instruction(avatar_bump: u8)]
pub struct Merge<'info> {
  #[account(
    init_if_needed,
    seeds = [sla_constants::PREFIX_LLAMA.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = avatar_bump,
    payer = payer, 
    space = sla_accounts::AvatarAccount::LEN,
  )]
  pub avatar: Account<'info, sla_accounts::AvatarAccount>,
  
  pub avatar_mint: Account<'info, anchor_spl::token::Mint>,
  
  #[account(mut)]
  pub trait_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = payer,
  )]
  pub avatar_token: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(
    mut,
    associated_token::mint = trait_mint,
    associated_token::authority = payer,
  )]
  pub trait_token: Account<'info, anchor_spl::token::TokenAccount>,
  
  #[account(mut)]
  pub avatar_metadata: AccountInfo<'info>,

  #[account(mut)]
  pub trait_metadata: AccountInfo<'info>,

  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    mut,
    constraint = assert_address(combine_authority.key, sla_constants::COMBINE_AUTHORITY_WALLET)
  )]
  pub combine_authority: Signer<'info>,

  #[account(address = anchor_spl::token::ID)]
  pub token_program: AccountInfo<'info>,

  #[account(address = mpl_token_metadata::ID)]
  pub metadata_program: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(treasury_bump: u8, asset_id: u8)]
pub struct MintIdCard<'info> {
  #[account(
    mut,
    constraint = sla_fungible_token::assert_mint_address(&mint.key(), asset_id) @ SlaErrors::InvalidPubkey
  )]
  pub mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = mint,
    associated_token::authority = user,
  )]
  pub ata: Account<'info, anchor_spl::token::TokenAccount>,

  // This is the person who is minting
  pub user: AccountInfo<'info>,

  // This is the SLA Treasury PDA
  #[account(
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(
    constraint = assert_address(&hay_mint.key(), sla_constants::HAY_TOKEN_MINT)
      @ SlaErrors::InvalidPubkey
  )]
  pub hay_mint: Account<'info, anchor_spl::token::Mint>,

  // This is the user's $HAY ATA
  #[account(
    mut,
    associated_token::mint = hay_mint,
    associated_token::authority = user,
  )]
  pub hay_user_ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(
    mut,
    constraint = assert_address(hay_treasury_ata.key, sla_constants::HAY_TREASURY_WALLET_ATA)
      @ SlaErrors::InvalidPubkey
  )]
  pub hay_treasury_ata: AccountInfo<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  treasury_bump: u8, 
  ranking_bump: u8, 
  badge_supply_counter_bump: u8,
  asset_id: u8,
)]
pub struct MintBadge<'info> {
  #[account(
    mut,
    constraint = sla_fungible_token::assert_mint_address(&mint.key(), asset_id) 
      @ SlaErrors::InvalidPubkey
  )]
  pub mint: Box<Account<'info, anchor_spl::token::Mint>>,

  #[account(
    init_if_needed,
    payer = user,
    associated_token::mint = mint,
    associated_token::authority = user,
  )]
  pub ata: Box<Account<'info, anchor_spl::token::TokenAccount>>,

  // This is the person who is minting
  pub user: Signer<'info>,

  // This is the SLA Treasury PDA
  #[account(
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(
    constraint = assert_address(&hay_mint.key(), sla_constants::HAY_TOKEN_MINT)
      @ SlaErrors::InvalidPubkey
  )]
  pub hay_mint: Box<Account<'info, anchor_spl::token::Mint>>,

  // This is the user's $HAY ATA
  #[account(
    mut,
    associated_token::mint = hay_mint,
    associated_token::authority = user,
  )]
  pub hay_user_ata: Box<Account<'info, anchor_spl::token::TokenAccount>>,

  #[account(
    mut,
    constraint = assert_address(hay_treasury_ata.key, sla_constants::HAY_TREASURY_WALLET_ATA)
      @ SlaErrors::InvalidPubkey
  )]
  pub hay_treasury_ata: AccountInfo<'info>,
  
  pub avatar_mint: Box<Account<'info, anchor_spl::token::Mint>>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = user,
  )]
  pub avatar_token: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub avatar_metadata: AccountInfo<'info>,

  #[account(
    init_if_needed,
    seeds = [sla_constants::PREFIX_RANKING.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = ranking_bump,
    payer = user, 
    space = sla_accounts::Ranking::LEN,
  )]
  pub ranking: Box<Account<'info, sla_accounts::Ranking>>,

  #[account(
    mut,
    seeds = [sla_constants::PREFIX_BADGE_POT.as_bytes()],
    bump = badge_supply_counter_bump,
  )]
  pub badge_supply_counter: Account<'info, sla_accounts::BadgeSupplyCounter>,

  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
  pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction()]
pub struct ChangeAlias<'info> {  
  pub avatar_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = payer,
  )]
  pub avatar_token: Account<'info, anchor_spl::token::TokenAccount>,
  
  #[account(mut)]
  pub avatar_metadata: AccountInfo<'info>,

  #[account(
    mut,
    constraint = assert_address(&id_card_mint.key(), sla_constants::ID_CARD_MINT)
      @ SlaErrors::InvalidPubkey
  )]
  pub id_card_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    mut,
    associated_token::mint = id_card_mint,
    associated_token::authority = payer,
  )]
  pub id_card_ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    mut,
    constraint = assert_address(combine_authority.key, sla_constants::COMBINE_AUTHORITY_WALLET)
  )]
  pub combine_authority: Signer<'info>,

  #[account(address = anchor_spl::token::ID)]
  pub token_program: AccountInfo<'info>,

  #[account(address = mpl_token_metadata::ID)]
  pub metadata_program: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  ranking_bump: u8,
  asset_id: u8,
)]
pub struct MergeBadge<'info> {  
  pub avatar_mint: Box<Account<'info, anchor_spl::token::Mint>>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = payer,
  )]
  pub avatar_token: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub avatar_metadata: AccountInfo<'info>,

  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    mut,
    constraint = sla_fungible_token::assert_mint_address(&badge_mint.key(), asset_id) 
      @ SlaErrors::InvalidPubkey
  )]
  pub badge_mint: Box<Account<'info, anchor_spl::token::Mint>>,

  #[account(
    mut,
    associated_token::mint = badge_mint,
    associated_token::authority = payer,
  )]
  pub badge_ata: Box<Account<'info, anchor_spl::token::TokenAccount>>,

  #[account(
    init_if_needed,
    seeds = [sla_constants::PREFIX_RANKING.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = ranking_bump,
    payer = payer, 
    space = sla_accounts::Ranking::LEN,
  )]
  pub ranking: Box<Account<'info, sla_accounts::Ranking>>,

  #[account(
    mut,
    constraint = assert_address(combine_authority.key, sla_constants::COMBINE_AUTHORITY_WALLET)
  )]
  pub combine_authority: Signer<'info>,

  #[account(address = anchor_spl::token::ID)]
  pub token_program: AccountInfo<'info>,

  #[account(address = mpl_token_metadata::ID)]
  pub metadata_program: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// #[instruction(badge_supply_counter_bump: u8)]
// pub struct InitBadgeSupplyCounter<'info> {
//   #[account(
//     init_if_needed,
//     seeds = [sla_constants::PREFIX_BADGE_POT.as_bytes()],
//     bump = badge_supply_counter_bump,
//     payer = authority,
//     space = sla_accounts::BadgeSupplyCounter::LEN,
//   )]
//   pub badge_supply_counter: Account<'info, sla_accounts::BadgeSupplyCounter>,

//   #[account(
//     constraint = assert_address(authority.key, sla_constants::COMBINE_AUTHORITY_WALLET)
//   )]
//   pub authority: Signer<'info>,

//   pub system_program: Program<'info, System>,
// }