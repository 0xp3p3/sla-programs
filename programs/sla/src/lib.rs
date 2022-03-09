use anchor_lang::{prelude::*};
use anchor_spl;
use mpl_token_metadata;
use solana_program;
use solana_program::program::{invoke_signed, invoke};

mod sla_accounts;
mod sla_errors;
mod utils;
mod sla_metadata;
mod sla_token;
mod sla_whitelist;
mod sla_hay;
mod sla_edition;
mod sla_constants;
mod sla_collection;
mod sla_pay;

use sla_errors::SlaErrors;
use utils::{assert_address, verify_avatar};

declare_id!("FX4vcKmc35gcU5hhqLQ9g5x7yHNY76Fve9yMHQU3uGLY");

// const ARWEAVE_WALLET: &[u8] = b"JDpq9RP9zUdVShvwwp2DK8orxU8e73SDMsQiYnsK87ga";

#[program]
pub mod sla {
    use super::*;

    pub fn create_collection(
      ctx: Context<CreateCollection>,
      treasury_bump: u8,
      name: String,
      symbol: String,
      uri: String, 
      seller_fee_basis_points: u16,
    ) -> ProgramResult {

      let metadata = ctx.accounts.metadata.to_account_info();
      let mint = ctx.accounts.collection_mint.to_account_info();
      let ata = ctx.accounts.collection_ata.to_account_info();
      let master_edition = ctx.accounts.collection_master_edition.to_account_info();
      let treasury = ctx.accounts.treasury.to_account_info();
      let creator = ctx.accounts.creator.to_account_info();
      let system_program = ctx.accounts.system_program.to_account_info();
      let rent_sysvar = ctx.accounts.rent.to_account_info();
      let token_program = ctx.accounts.token_program.to_account_info();
      let mpl_token_metadata_program = ctx.accounts.mpl_token_metadata_program.to_account_info();

      // Create the Metadata account for the Collection
      msg!("Creating Collection Metadata");
      sla_metadata::create_metadata_account(
        metadata.clone(),
        mint.clone(),
        treasury.clone(),
        creator.clone(),
        name,
        symbol,
        uri, 
        seller_fee_basis_points,
        system_program.clone(),
        rent_sysvar.clone(),
        mpl_token_metadata_program.clone(),
        None,
        treasury_bump,
      )?;

      // Mint a single token for the Master Edition
      msg!("Minting unique Master Edition token");
      sla_token::mint_edition_unique_token(
        mint.clone(), 
        ata,
        creator.clone(),  // current ATA account owner
        treasury.clone(),  // mint authority
        treasury.clone(),  // final ATA account owner
        token_program.clone(),
        treasury_bump
      )?;

      // Create a Master Edition for the Collection
      msg!("Creating Collection Master Edition");
      sla_edition::create_master_edition(
        mint.clone(),
        metadata.clone(),
        master_edition.clone(),
        creator.clone(),  // update authority, payer
        treasury.clone(),  // mint authority
        token_program.clone(),
        system_program.clone(),
        rent_sysvar.clone(),
        mpl_token_metadata_program.clone(),
        treasury_bump,
        Some(0),
      )?;

      msg!("CreateCollection instruction finished");

      Ok(())
    }

    pub fn create_master_edition(
      ctx: Context<CreateMasterEdition>,
      treasury_bump: u8,
      name: String,
      symbol: String,
      uri: String, 
      seller_fee_basis_points: u16,
    ) -> ProgramResult {

      let metadata = ctx.accounts.metadata.to_account_info();
      let mint = ctx.accounts.mint.to_account_info();
      let ata = ctx.accounts.ata.to_account_info();
      let master_edition = ctx.accounts.master_edition.to_account_info();
      let treasury = ctx.accounts.treasury.to_account_info();
      let creator = ctx.accounts.creator.to_account_info();
      let system_program = ctx.accounts.system_program.to_account_info();
      let rent_sysvar = ctx.accounts.rent.to_account_info();
      let token_program = ctx.accounts.token_program.to_account_info();
      let mpl_token_metadata_program = ctx.accounts.mpl_token_metadata_program.to_account_info();

      let collection_mint = ctx.accounts.collection_mint.to_account_info();
      let collection_metadata = ctx.accounts.collection_metadata.to_account_info();
      let collection_master_edition = ctx.accounts.collection_master_edition.to_account_info();

      // Collection
      let collection = mpl_token_metadata::state::Collection {
        verified: false,
        key: ctx.accounts.collection_mint.key(),
      };

      // Create the Metadata account
      msg!("Creating Master Edition Metadata");
      sla_metadata::create_metadata_account(
        metadata.clone(),
        mint.clone(),
        treasury.clone(),
        creator.clone(),
        name,
        symbol,
        uri, 
        seller_fee_basis_points,
        system_program.clone(),
        rent_sysvar.clone(),
        mpl_token_metadata_program.clone(),
        Some(collection),
        treasury_bump,
      )?;

      // Mint a single token for the Master Edition
      msg!("Minting unique Master Edition token");
      sla_token::mint_edition_unique_token(
        mint.clone(), 
        ata,
        creator.clone(),  // current ATA account owner
        treasury.clone(),  // mint authority
        treasury.clone(),  // final ATA account owner
        token_program.clone(),
        treasury_bump
      )?;

      // Create the Master Edition
      msg!("Creating Master Edition");
      sla_edition::create_master_edition(
        mint.clone(),
        metadata.clone(),
        master_edition.clone(),
        creator.clone(),
        treasury.clone(),
        token_program,
        system_program.clone(),
        rent_sysvar.clone(),
        mpl_token_metadata_program.clone(),
        treasury_bump,
        None,  // max supply
      )?;

      // Verify the Collection
      msg!("Verifying the collection");
      sla_collection::verify_collection(
        metadata,
        creator,
        collection_mint,
        collection_metadata,
        collection_master_edition,
        treasury_bump
      )?;

      msg!("CreateMasterEdition instruction finished!");
      Ok(())
    }

    pub fn mint_edition(
      ctx: Context<MintEdition>,
      treasury_bump: u8,
      edition_number: u64,
    ) -> ProgramResult {

      let new_metadata = ctx.accounts.new_metadata.to_account_info();
      let new_edition = ctx.accounts.new_edition.to_account_info();
      let new_mint = ctx.accounts.new_mint.to_account_info();
      let new_ata = ctx.accounts.new_ata.to_account_info();
      let edition_marker = ctx.accounts.edition_marker.to_account_info();
      let master_edition = ctx.accounts.master_edition.to_account_info();
      let master_metadata = ctx.accounts.master_metadata.to_account_info();
      let master_mint = ctx.accounts.master_mint.to_account_info();
      let master_ata = ctx.accounts.master_ata.to_account_info();
      let user = ctx.accounts.user.to_account_info();
      let treasury = ctx.accounts.treasury.to_account_info();
      let system_program = ctx.accounts.system_program.to_account_info();
      let rent_sysvar = ctx.accounts.rent.to_account_info();
      let token_program = ctx.accounts.token_program.to_account_info();

      // $HAY-related accounts
      let hay_mint = ctx.accounts.hay_mint.to_account_info();
      let hay_user_ata = ctx.accounts.hay_user_ata.to_account_info();

      // Check that the Llama belongs to our collection
      let llama_mint = ctx.accounts.llama_mint.key();
      let llama_ata = ctx.accounts.llama_ata.clone();
      let llama_metadata = ctx.accounts.llama_metadata.to_account_info();
      verify_avatar(llama_mint, llama_ata, user.key(), &llama_metadata)?;

      // Mint a single Token from the new mint 
      sla_token::mint_edition_unique_token(
        new_mint.clone(), 
        new_ata, 
        user.clone(),  // ATA account owner
        treasury.clone(),  // mint authority
        user.clone(),  // final ATA account owner
        token_program.clone(),
        treasury_bump
      )?;

      // Mint the new Edition
      sla_edition::mint_edition_from_master_edition(
        new_metadata.clone(),
        new_edition,
        master_edition,
        new_mint,
        edition_marker,
        treasury.clone(),
        user.clone(),
        master_ata,
        master_metadata,
        master_mint.clone(),
        token_program.clone(),
        system_program,
        rent_sysvar,
        edition_number,
        treasury_bump
      )?;

      // Set `primary_sale_happened` to True
      sla_metadata::set_primary_sale_happened(
        new_metadata,  // metadata account to update
        treasury,  // update authority
        treasury_bump
      )?;

      // Pay for the new edition
      sla_pay::pay_for_edition_mint(
        master_mint.key,
        hay_mint,
        hay_user_ata,
        user,
        token_program,
      )?;

      Ok(())
    }

    pub fn mint_unlimited_hay(
      ctx: Context<MintHayUnlimited>,
      treasury_bump: u8,
      amount: u64,
    ) -> ProgramResult {

      let signer_seeds = &[&[sla_constants::PREFIX_TREASURY.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];

      // Mint $HAY
      sla_token::mint_tokens(
        ctx.accounts.hay_mint.to_account_info(),
        ctx.accounts.hay_ata.to_account_info(),
        ctx.accounts.treasury.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        Some(signer_seeds),
        amount,
      )
    }

    pub fn mint_hay(
      ctx: Context<MintHay>,
      treasury_bump: u8,
      llama_bump: u8,
    ) -> ProgramResult {

      let llama = &mut ctx.accounts.llama;

      // Check if this Llama is allowed to mint Hay at this time of day,
      llama.mint_hay(ctx.accounts.clock.unix_timestamp)?;

      // Mint some Hay
      sla_token::deprecated_mint_hay(
        ctx.accounts.hay_mint.to_account_info(),
        ctx.accounts.hay_ata.to_account_info(),
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
#[instruction(treasury_bump: u8)]
pub struct CreateCollection<'info> {
  #[account(mut)]
  pub metadata: AccountInfo<'info>,

  #[account(
    init, 
    payer = creator, 
    mint::decimals = 0,
    mint::authority = treasury,
  )]
  pub collection_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    init,
    payer = creator,
    associated_token::mint = collection_mint,
    associated_token::authority = creator,
  )]
  pub collection_ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub collection_master_edition: AccountInfo<'info>,

  #[account(
    init_if_needed,
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
    space = 8,
    payer = creator,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(
    mut,
    constraint = assert_address(creator.key, sla_constants::COLLECTIONS_CREATOR_WALLET)
      @ SlaErrors::InvalidCreatorPubkey
  )]
  pub creator: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
  #[account(address = mpl_token_metadata::ID)]
  pub mpl_token_metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(treasury_bump: u8)]
pub struct CreateMasterEdition<'info> {
  #[account(
    init, 
    payer = creator, 
    mint::decimals = 0,
    mint::authority = treasury,
  )]
  pub mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    init,
    payer = creator,
    associated_token::mint = mint,
    associated_token::authority = creator,
  )]
  pub ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub metadata: AccountInfo<'info>,

  #[account(mut)]
  pub master_edition: AccountInfo<'info>,

  pub collection_mint: Account<'info, anchor_spl::token::Mint>,
  pub collection_metadata: AccountInfo<'info>,
  pub collection_master_edition: AccountInfo<'info>,

  #[account(
    init_if_needed,
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
    space = 8,
    payer = creator,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(
    mut,
    constraint = assert_address(creator.key, sla_constants::COLLECTIONS_CREATOR_WALLET)
      @ SlaErrors::InvalidCreatorPubkey
  )]
  pub creator: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
  #[account(address = mpl_token_metadata::ID)]
  pub mpl_token_metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(treasury_bump: u8)]
pub struct MintEdition<'info> {
  // Additional checks on the Llama are performed later
  pub llama_mint: Account<'info, anchor_spl::token::Mint>,
  #[account(
    associated_token::mint = llama_mint,
    associated_token::authority = user,
  )]
  pub llama_ata: Account<'info, anchor_spl::token::TokenAccount>,
  pub llama_metadata: AccountInfo<'info>,

  // $HAY accounts
  #[account(
    constraint = assert_address(&hay_mint.key(), sla_constants::HAY_TOKEN_MINT)
  )]
  pub hay_mint: Account<'info, anchor_spl::token::Mint>,
  
  #[account(
    associated_token::mint = hay_mint,
    associated_token::authority = user,
  )]
  pub hay_user_ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub new_metadata: AccountInfo<'info>,

  #[account(mut)]
  pub new_edition: AccountInfo<'info>,
  
  #[account(
    init,
    payer = user,
    mint::decimals = 0,
    mint::authority = treasury,
  )]
  pub new_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    init,
    payer = user, 
    associated_token::mint = new_mint,
    associated_token::authority = user,
  )]
  pub new_ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(mut)]
  pub edition_marker: AccountInfo<'info>,

  #[account(mut)]
  pub master_edition: AccountInfo<'info>,

  // Checks will be performed by mpl_token_metadata_program
  pub master_metadata: AccountInfo<'info>,
  
  // Checks will be performed by mpl_token_metadata_program
  pub master_mint: Account<'info, anchor_spl::token::Mint>,

  // Checks will be performed by mpl_token_metadata_program
  pub master_ata: Account<'info, anchor_spl::token::TokenAccount>,
  
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
  )]
  pub treasury: AccountInfo<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
  #[account(address = mpl_token_metadata::ID)]
  pub mpl_token_metadata_program: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(treasury_bump: u8)]
pub struct MintHayUnlimited<'info> {
  #[account(mut)]
  pub hay_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = hay_mint,
    associated_token::authority = user,
  )]
  pub hay_ata: Account<'info, anchor_spl::token::TokenAccount>,

  // This is the person to whom we are sending $HAY
  pub user: AccountInfo<'info>,

  // This is the SLA Treasury PDA
  #[account(
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(
    mut,
    constraint = assert_address(authority.key, sla_constants::HAY_TREASURY_WALLET)
      @ SlaErrors::SignerIsNotHayTreasury
  )]
  pub authority: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(treasury_bump: u8, llama_bump: u8)]
pub struct MintHay<'info> {
  #[account(mut)]
  pub hay_mint: Account<'info, anchor_spl::token::Mint>,
  pub llama_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    init_if_needed,
    payer = fee_payer,
    associated_token::mint = hay_mint,
    associated_token::authority = user,
  )]
  pub hay_ata: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(
    associated_token::mint = llama_mint,
    associated_token::authority = user,
  )]
  pub llama_ata: Account<'info, anchor_spl::token::TokenAccount>,

  // This is the PDA associated with the Llama NFT
  #[account(
    init_if_needed,
    seeds = [sla_constants::PREFIX_LLAMA.as_bytes(), &llama_mint.key().to_bytes()],
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
    seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
    bump = treasury_bump,
  )]
  pub treasury: AccountInfo<'info>,

  #[account(mut)]
  pub fee_payer: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,
  pub clock: Sysvar<'info, Clock>,
  pub token_program: Program<'info, anchor_spl::token::Token>,
  pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
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
    seeds = [sla_constants::PREFIX_LLAMA.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = avatar_bump,
    payer = payer, 
    space = sla_accounts::AvatarAccount::LEN,
  )]
  pub avatar: Account<'info, sla_accounts::AvatarAccount>,
  
  pub avatar_mint: Account<'info, anchor_spl::token::Mint>,
  pub trait_mint: Account<'info, anchor_spl::token::Mint>,

  #[account(
    associated_token::mint = avatar_mint,
    associated_token::authority = payer,
  )]
  pub avatar_token: Account<'info, anchor_spl::token::TokenAccount>,

  #[account(
    associated_token::mint = trait_mint,
    associated_token::authority = payer,
  )]
  pub trait_token: Account<'info, anchor_spl::token::TokenAccount>,
  
  #[account(mut)]
  pub payer: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(master_bump: u8, avatar_bump: u8)]
pub struct Merge<'info> {
  #[account(
    mut,
    seeds = [sla_constants::PREFIX_LLAMA.as_bytes(), &avatar_mint.key().to_bytes()],
    bump = avatar_bump,
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
  pub payer: Signer<'info>,

  #[account(mut)]
  pub avatar_metadata: AccountInfo<'info>,

  #[account(
    seeds = [sla_constants::PREFIX_MASTER.as_bytes()],
    bump = master_bump,
  )]
  pub sla_master_pda: AccountInfo<'info>,

  // The wallet paying for Arweave upload transactions
  #[account(mut)]
  pub arweave_wallet: AccountInfo<'info>,

  #[account(address = anchor_spl::token::ID)]
  pub token_program: AccountInfo<'info>,

  #[account(address = mpl_token_metadata::ID)]
  pub metadata_program: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}
