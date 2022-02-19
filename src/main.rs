use spl_token::{instruction::initialize_mint};
use solana_program::{
  program::{invoke},
  account_info::{next_account_info},
};

pub fn init_mint(
  accounts: &[AccountInfo],
  decimals: u8,
) -> ProgramResult {

  let accounts_iter = &mut accounts.iter();
  let spl_token_account = next_account_info(accounts_iter)?;
  let mint_account = next_account_info(accounts_iter)?;
  let mint_authority = next_account_info(accounts_iter)?;
  let freeze_authority = next_account_info(accounts_iter)?;

  //create instruction
  let initialize_mint_instruction = initialize_mint(
    spl_token_account.key,
    mint_account.key,
    mint_authority.key,
    freeze_authority.key,
    decimals;
  )?;

  //normal case for token transfer with a user as the authority
  invoke(
    &initialize_mint_instruction,
    &[
      spl_token_account.clone(),
      mint_account.clone(),
      mint_authority.clone(),
      freeze_authority.clone(),
    ],
    )?;
  Ok(())
}