use anchor_lang::prelude::*;

#[error]
pub enum SlaErrors {
  #[msg("The creator pubkey is not valid")]
  InvalidCreatorPubkey,

  /*** GENERAL ERRROS ***/
  
  #[msg("Invalid hard-coded pubkey")]
  InvalidPubkey,

  /*** CHECK AVATAR OWNERSHIP ***/

  #[msg("ATA's amount is not 1")]
  AtaAmountIsNotOne,

  #[msg("Mint and ATA do not match")]
  MintAndAtaMismatch,
  
  #[msg("The user account is not owner of the Mint")]
  UserDoesNotOwnMint,
  
  #[msg("The user is not the owner of the token specified")]
  TokenPDAMismatch,
  
  #[msg("Avatar is not part of the collection")]
  AvatarNotInCollection,
  
  #[msg("Candy machine creator is not valid or unverified")]
  CreatorInvalid,

  /*** CHECK TRAIT OWNERSHIP ***/

  #[msg("Trait is not part of a verified collection")]
  TraitNotInVerifiedCollection,

  #[msg("Trait is not part of any trait collection")]
  TraitCollectionUnknown,


  /*** HAY-RELATED ERRORS  ***/

  #[msg("Signer is not the $HAY treasury wallet")]
  SignerIsNotHayTreasury,

  //**************

  /*** MINTING FUNGIBLE ASSET ERRORS ***/

  #[msg("Assset ID does not match a Badge")]
  AssetIDIsNotBadge,

  #[msg("Agent does not have the required rank to mint this badge")]
  NotAllowedToMintBadge,

  #[msg("Agent has already minted the next badge")]
  NextBadgeAlreadyMinted,

  #[msg("Max supply has been reached")]
  AssetMaxSupplyReached,

  //**************

  /*** MINTING NEW EDITION ERRORS ***/

  #[msg("Master Edition not recognised")]
  MasterEditionNotRecognised,

  #[msg("Token to be minted not recognised")]
  TokenToMintNotRecognised,

  //**************


  #[msg("The Avatar PDA is not an Avatar account")]
  PDAIsNotAnAvatar,

  #[msg("The Trait PDA is not a Trait account")]
  PDAIsNotATrait,

  #[msg("The Trait type specified is not valid")]
  TraitTypeInvalid,

  #[msg("The Avatar PDA has already been initialized")]
  AvatarAlreadyInitialized,

  #[msg("The Avatar PDA has not been initialized yet")]
  AvatarPDANotInitialized,

  #[msg("The Trait specified cannot be merged")]
  MergeCheckFailed,

  #[msg("The Trait cannot be merged because the Avatar has not been initialized")]
  MergeCheckFailedBecauseAvatarNotInitialized,

  #[msg("The specified arweave_wallet account does not match the stored pubkey")]
  ArweaveAccountMismatch,

  #[msg("The trait cannot be minted by this avatar")]
  AvatarCannotMintTrait,

}
