use anchor_lang::prelude::*;
use anchor_spl;

pub fn transfer<'info>(from: AccountInfo<'info>, to: AccountInfo<'info>, lambports: u64) -> Result<(), ProgramError> {

  let instruction = solana_program::system_instruction::transfer(&from.key(), &to.key(), lambports);

  solana_program::program::invoke(
    &instruction,
    &[from, to]
  )
}

pub fn verify_avatar<'info>(
  mint: AccountInfo<'info>,
  ata: AccountInfo<'info>,
) -> Result<(), ProgramError> {

  Ok(())
}