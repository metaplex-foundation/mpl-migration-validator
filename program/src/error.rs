use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MigrationError {
    /// 0
    #[error("Metadata does not match mint account")]
    MetadataMintMistmatch,
    /// 1
    #[error("Metadata did not deserialize correctly")]
    InvalidMetadata,
    /// 2
    #[error("Authority does not match the authority on the account")]
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
    /// 6
    #[error("Incorrect program owner for migration state account")]
    IncorrectProgramOwner,
    /// 7
    #[error("Overflow error")]
    Overflow,
    /// 8
    #[error("Failed to build Migrate instruction")]
    InvalidInstruction,
    /// 9
    #[error("No rule set provided")]
    NoRuleSet,
    /// 10
    #[error("Program signer account derivation is in correct")]
    InvalidSignerDerivation,
    /// 11
    #[error("Program signer is already initialized")]
    AlreadyInitialized,
    /// 12
    #[error("Invalid delegate")]
    InvalidDelegate,
    /// 13
    #[error("This feature is currently disabled")]
    FeatureDisabled,
    /// 14
    #[error("Invalid delegate record derivation")]
    InvalidDelegateRecordDerivation,
    /// 15
    #[error("Collection mint does not match state account")]
    CollectionMintMismatch,
    /// 16
    #[error("Migration state account is locked")]
    MigrationLocked,
}

impl PrintProgramError for MigrationError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MigrationError> for ProgramError {
    fn from(e: MigrationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MigrationError {
    fn type_of() -> &'static str {
        "Migrate Error"
    }
}
