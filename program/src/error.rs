use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MigrateError {
    /// 0
    #[error("Metadata does not match mint account")]
    MetadataMintMistmatch,
    /// 1
    #[error("Metadata did not deserialize correctly")]
    InvalidMetadata,
    /// 2
    #[error("Authority does not match update authority on metadata")]
    InvalidAuthority,
    /// 3
    #[error("Migration state account derivation is in correct")]
    InvalidStateDerivation,
    /// 4
    #[error("Migration state did not deserialize correctly")]
    InvalidStateDeserialization,
    /// 5
    #[error("Cannot close while migration is in progress")]
    MigrationInProgress,
}

impl PrintProgramError for MigrateError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MigrateError> for ProgramError {
    fn from(e: MigrateError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MigrateError {
    fn type_of() -> &'static str {
        "Migrate Error"
    }
}
