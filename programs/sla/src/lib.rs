use anchor_lang::{prelude::*};
use anchor_spl;
use mpl_token_metadata;

mod sla_accounts;
mod sla_errors;
mod utils;
mod sla_metadata;
mod sla_token;
mod sla_edition;
mod sla_constants;
mod sla_collection;
mod sla_fungible_token;

use sla_errors::SlaErrors;
use utils::{assert_address, verify_avatar, verify_trait};

declare_id!("GUSxqUfUdqchfErA3DrW1jNVJKGdMpxt71AeDkJJtG5R");


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
        utils::str_to_pubkey(sla_constants::COMMUNITY_WALLET),
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
        treasury.key(),  // final ATA account owner
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
        utils::str_to_pubkey(sla_constants::COMMUNITY_WALLET),
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
        treasury.key(),  // final ATA account owner
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

    // pub fn mint_edition(
    //   ctx: Context<MintEdition>,
    //   master_mint: Pubkey,
    //   treasury_bump: u8,
    //   edition_number: u64,
    //   edition_type: u8,
    // ) -> ProgramResult {

    //   // Check the addresses given are correct
    //   // assert!(assert_address(&ctx.accounts.hay_mint.key(), sla_constants::HAY_TOKEN_MINT));
    //   assert!(assert_address(ctx.accounts.hay_treasury_ata.key, sla_constants::HAY_TREASURY_WALLET_ATA));

    //   let user = ctx.accounts.user.to_account_info();
    //   let treasury = ctx.accounts.treasury.to_account_info();

    //   // Mint a single Token from the new mint 
    //   sla_token::mint_edition_unique_token(
    //     ctx.accounts.new_mint.to_account_info().clone(), 
    //     ctx.accounts.new_ata.to_account_info(), 
    //     user.clone(),  // ATA account owner
    //     treasury.clone(),  // mint authority
    //     user.key(),  // final ATA account owner
    //     ctx.accounts.token_program.to_account_info().clone(),
    //     treasury_bump
    //   )?;

    //   // Mint the new Edition
    //   sla_edition::mint_edition_from_master_edition(
    //     ctx.accounts.new_metadata.to_account_info().clone(),
    //     ctx.accounts.new_edition.to_account_info(),
    //     ctx.accounts.master_edition.to_account_info(),
    //     ctx.accounts.new_mint.to_account_info(),
    //     ctx.accounts.edition_marker.to_account_info(),
    //     treasury.clone(),
    //     user.clone(),
    //     ctx.accounts.master_ata.to_account_info(),
    //     ctx.accounts.master_metadata.to_account_info(),
    //     master_mint,
    //     ctx.accounts.token_program.to_account_info().clone(),
    //     ctx.accounts.system_program.to_account_info(),
    //     ctx.accounts.rent.to_account_info(),
    //     edition_number,
    //     treasury_bump
    //   )?;

    //   // Set `primary_sale_happened` to True
    //   sla_metadata::set_primary_sale_happened(
    //     ctx.accounts.new_metadata.to_account_info(),  // metadata account to update
    //     treasury,  // update authority
    //     treasury_bump
    //   )?;

    //   // Pay for the new edition
    //   sla_pay::pay_for_edition_mint(
    //     &master_mint,
    //     ctx.accounts.hay_user_ata.to_account_info(),
    //     ctx.accounts.hay_treasury_ata.to_account_info(),
    //     user,
    //     ctx.accounts.token_program.to_account_info(),
    //     edition_type,
    //   )?;

    //   Ok(())
    // }

    // pub fn mint_unlimited_hay(
    //   ctx: Context<MintHayUnlimited>,
    //   treasury_bump: u8,
    //   amount: u64,
    // ) -> ProgramResult {

    //   let signer_seeds = &[&[sla_constants::PREFIX_TREASURY.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];

    //   // Mint $HAY
    //   sla_token::mint_tokens(
    //     ctx.accounts.hay_mint.to_account_info(),
    //     ctx.accounts.hay_ata.to_account_info(),
    //     ctx.accounts.treasury.to_account_info(),
    //     ctx.accounts.token_program.to_account_info(),
    //     Some(signer_seeds),
    //     amount,
    //   )
    // }

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


    pub fn mint_fungible_asset(ctx: Context<MintFungibleAsset>, treasury_bump: u8, asset_id: u8) -> ProgramResult {
      msg!("Entering the MintFungibleAsset instruction");

      let mint = ctx.accounts.mint.to_account_info();
      let ata = ctx.accounts.ata.to_account_info();
      let treasury = ctx.accounts.treasury.to_account_info();
      let token_program = ctx.accounts.token_program.to_account_info();

      let signer_seeds = &[&[sla_constants::PREFIX_TREASURY.as_bytes(), bytemuck::bytes_of(&treasury_bump)][..]];
      
      let fungible_asset = sla_fungible_token::FungibleAsset::from_u8(asset_id);
      let price = fungible_asset.get_price();

      msg!("Minting a new {}", fungible_asset.to_string());
      sla_token::mint_tokens(mint, ata, treasury, token_program.clone(), Some(signer_seeds), 1)?;

      if fungible_asset != sla_fungible_token::FungibleAsset::ID_CARD {
        panic!("Only ID Cards can be minted at this point")
      }

      msg!("Transferring {} $HAY to treasury", price);
      sla_token::transfer_tokens(
        ctx.accounts.hay_user_ata.to_account_info(), 
        ctx.accounts.hay_treasury_ata.to_account_info(), 
        ctx.accounts.user.to_account_info(), 
        token_program, 
        u64::from(price),
      )?;
      
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

// #[derive(Accounts)]
// #[instruction(treasury_bump: u8)]
// pub struct MintEdition<'info> {
//   #[account(
//     constraint = assert_address(&hay_mint.key(), sla_constants::HAY_TOKEN_MINT)
//       @ SlaErrors::InvalidPubkey
//   )]
//   pub hay_mint: Box<Account<'info, anchor_spl::token::Mint>>,
  
//   #[account(
//     mut,
//     associated_token::mint = hay_mint,
//     associated_token::authority = user,
//   )]
//   pub hay_user_ata: Box<Account<'info, anchor_spl::token::TokenAccount>>,

//   // Address checked in instruction
//   #[account(
//     mut,
//     constraint = assert_address(hay_treasury_ata.key, sla_constants::HAY_TREASURY_WALLET_ATA)
//       @ SlaErrors::InvalidPubkey
//   )]
//   pub hay_treasury_ata: AccountInfo<'info>,

//   #[account(mut)]
//   pub new_metadata: AccountInfo<'info>,

//   #[account(mut)]
//   pub new_edition: AccountInfo<'info>,
  
//   #[account(
//     init,
//     payer = user,
//     mint::decimals = 0,
//     mint::authority = treasury,
//   )]
//   pub new_mint: Account<'info, anchor_spl::token::Mint>,

//   #[account(
//     init,
//     payer = user, 
//     associated_token::mint = new_mint,
//     associated_token::authority = user,
//   )]
//   pub new_ata: Account<'info, anchor_spl::token::TokenAccount>,

//   #[account(mut)]
//   pub edition_marker: AccountInfo<'info>,

//   #[account(mut)]
//   pub master_edition: AccountInfo<'info>,

//   // Checks will be performed by mpl_token_metadata_program
//   pub master_metadata: AccountInfo<'info>,

//   // Checks will be performed by mpl_token_metadata_program
//   pub master_ata: Box<Account<'info, anchor_spl::token::TokenAccount>>,
  
//   #[account(mut)]
//   pub user: Signer<'info>,

//   // #[account(
//   //   seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
//   //   bump = treasury_bump,
//   // )]
//   pub treasury: AccountInfo<'info>,

//   pub rent: Sysvar<'info, Rent>,
//   pub token_program: Program<'info, anchor_spl::token::Token>,
//   pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
//   #[account(address = mpl_token_metadata::ID)]
//   pub mpl_token_metadata_program: AccountInfo<'info>,
//   pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// #[instruction(treasury_bump: u8)]
// pub struct MintHayUnlimited<'info> {
//   #[account(mut)]
//   pub hay_mint: Account<'info, anchor_spl::token::Mint>,

//   #[account(
//     init_if_needed,
//     payer = authority,
//     associated_token::mint = hay_mint,
//     associated_token::authority = user,
//   )]
//   pub hay_ata: Account<'info, anchor_spl::token::TokenAccount>,

//   // This is the person to whom we are sending $HAY
//   pub user: AccountInfo<'info>,

//   // This is the SLA Treasury PDA
//   #[account(
//     seeds = [sla_constants::PREFIX_TREASURY.as_bytes()],
//     bump = treasury_bump,
//   )]
//   pub treasury: AccountInfo<'info>,

//   #[account(
//     mut,
//     constraint = assert_address(authority.key, sla_constants::HAY_TREASURY_WALLET)
//       @ SlaErrors::SignerIsNotHayTreasury
//   )]
//   pub authority: Signer<'info>,

//   pub rent: Sysvar<'info, Rent>,
//   pub token_program: Program<'info, anchor_spl::token::Token>,
//   pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
//   pub system_program: Program<'info, System>,
// }

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
pub struct MintFungibleAsset<'info> {
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