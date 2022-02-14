use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        borsh::try_from_slice_unchecked, // serilization deserilization
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
};

// => serlization vault binary
// => deri

use solana_program::program_error::ProgramError;
use std::convert::TryInto;

/// spl => erc
/// AMM, dspefi spl
/// erc 20

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct VaultID {
    pub id: i32,
    pub EthAddress: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, PartialOrd)]
pub struct keys {
    pub id: i32,
    pub Owner: Pubkey,
}

/// Instructions supported by the Fraction program.
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum VaultInstruction {
    InitVault(VaultID),
    UnlockVault(keys),
}

#[allow(clippy::too_many_arguments)]
pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],

    data_size: usize,
) -> Result<T, ProgramError> {
    // if (data[0] != data_type as u8 && data[0] != Key::Uninitialized as u8)
    //     || data.len() != data_size
    // {
    //     return Err(VaultError::DataTypeMismatch.into());
    // }

    let result: T = try_from_slice_unchecked(data)?;

    Ok(result)
}

#[allow(clippy::too_many_arguments)]
pub fn create_unlock_vault_instruction(
    program_id: Pubkey,
    authority: Pubkey,
    vault_id: i32,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(authority, false)],
        data: VaultInstruction::UnlockVault(keys {
            id: vault_id,
            Owner: authority,
        })
        .try_to_vec()
        .unwrap(),
    }
}

/// Creates an InitVault instruction
#[allow(clippy::too_many_arguments)]
pub fn create_init_vault_instruction(
    program_id: Pubkey,
    ERC20: &str,
    vault: Pubkey,
    vault_authority: Pubkey,
) -> Instruction {
    let mut counter = -1;
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(vault_authority, false),
        ],
        data: VaultInstruction::InitVault(VaultID {
            id: counter + 1,              // i32 0 1 2
            EthAddress: ERC20.to_owned(), // ox
        })
        .try_to_vec()
        .unwrap(),
    }
}
