use anchor_lang::prelude::*;
use mpl_token_metadata;

use crate::sla_constants;
use crate::utils::str_to_pubkey;
use crate::SlaErrors;


pub fn check_collection(
  collection_member: mpl_token_metadata::state::Metadata,
  expected: &Pubkey,
) -> bool {
  match &collection_member.collection {
    Some(collection) => {
      collection.key == *expected && collection.verified
    },
    None => false,
  }
}

// Returns the collection key and the trait ID if the trait is part of one of the trait collections
pub fn extract_trait_id(trait_metadata: mpl_token_metadata::state::Metadata) -> Result<(Pubkey, u8), SlaErrors> {

  match &trait_metadata.collection {
    Some(collection) => {
      let key = collection.key;

      if key == str_to_pubkey(sla_constants::SKIN_COLLECTION) { 
        Ok((str_to_pubkey(sla_constants::SKIN_COLLECTION), 1))
      } else if key == str_to_pubkey(sla_constants::CLOTHING_COLLECTION) { 
        Ok((str_to_pubkey(sla_constants::CLOTHING_COLLECTION), 2))
      } else if key == str_to_pubkey(sla_constants::EYES_COLLECTION) { 
        Ok((str_to_pubkey(sla_constants::EYES_COLLECTION), 3))
      } else if key == str_to_pubkey(sla_constants::HAT_COLLECTION) { 
        Ok((str_to_pubkey(sla_constants::HAT_COLLECTION), 4))
      } else if key == str_to_pubkey(sla_constants::MOUTH_COLLECTION) { 
        Ok((str_to_pubkey(sla_constants::MOUTH_COLLECTION), 5))
      } else { 
        Err(SlaErrors::TraitCollectionUnknown)
      }
    },
    None => Err(SlaErrors::TraitNotInVerifiedCollection)
  }
}