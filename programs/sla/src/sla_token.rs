use anchor_lang::prelude::*;
use anchor_spl;


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

pub fn transfer_tokens<'info>(
  from: AccountInfo<'info>,
  to: AccountInfo<'info>,
  authority: AccountInfo<'info>,
  token_program: AccountInfo<'info>,
  amount: u64,
) -> ProgramResult {

  let accounts = anchor_spl::token::Transfer {
    from: from,
    to: to,
    authority: authority,
  };

  let cpi_ctx = CpiContext::new(token_program, accounts);

  anchor_spl::token::transfer(cpi_ctx, amount)
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
