use {
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo,
        borsh::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack},
        pubkey::Pubkey,
        system_instruction,
        sysvar::{rent::Rent, Sysvar},
    },
    std::convert::TryInto,
};

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
