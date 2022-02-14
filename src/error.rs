//! Error types

use {
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the Vault program.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum VaultError {
    #[error("Invalid Authority")]
    InvalidAuthority,

    #[error("Derived key invalid")]
    InvalidID,
}

impl PrintProgramError for VaultError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VaultError {
    fn type_of() -> &'static str {
        "Vault Error"
    }
}
