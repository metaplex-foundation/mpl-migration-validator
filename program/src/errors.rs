use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

const MIGRATION_ERROR_START: isize = 25;
const VALIDATOR_ERROR_START: isize = 50;
const DESERIALIZATION_ERROR_START: isize = 75;

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum GeneralError {
    #[error("Overflow error")]
    Overflow,

    #[error("Failed to build Migrate instruction")]
    InvalidInstruction,

    #[error("No rule set provided")]
    NoRuleSet,

    #[error("This feature is currently disabled")]
    FeatureDisabled,

    #[error("Invalid unlock method")]
    InvalidUnlockMethod,
}

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MigrationError {
    #[error("Cannot perform this action while migration is in progress")]
    MigrationInProgress = MIGRATION_ERROR_START,

    #[error("Cannot be closed after migration has completed")]
    MigrationAlreadyCompleted,

    #[error("Program signer is already initialized")]
    AlreadyInitialized,

    #[error("Migration state account is locked")]
    MigrationLocked,

    #[error("Immutable metadata cannot be migrated")]
    ImmutableMetadata,

    #[error("Incorrect freeze authority")]
    IncorrectFreezeAuthority,

    #[error("Incorrect token standard: must be NonFungible")]
    IncorrectTokenStandard,
}

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum ValidationError {
    #[error("Metadata does not match mint account")]
    MetadataMintMistmatch = VALIDATOR_ERROR_START,

    #[error("Token does not match the mint account")]
    TokenMintMismatch,

    #[error("Collection mint does not match stored value")]
    CollectionMintMismatch,

    #[error("Authority does not match the authority on the account")]
    InvalidAuthority,

    #[error("No collection found on item")]
    CollectionNotFound,

    #[error("Item is not a verified member of the collection")]
    NotCollectionMember,

    #[error("Invalid token standard")]
    InvalidTokenStandard,

    #[error("Missing token standard")]
    MissingTokenStandard,

    #[error("The metadata derivation does not match the mint account")]
    InvalidMetadataDerivation,

    #[error("The edition derivation does not match the mint account")]
    InvalidEditionDerivation,

    #[error("Migration state account derivation is in correct")]
    InvalidMigrationStateDerivation,

    #[error("Program signer account derivation is incorrect")]
    InvalidSignerDerivation,

    #[error("Invalid delegate record derivation")]
    InvalidDelegateRecordDerivation,

    #[error("Invalid delegate")]
    InvalidDelegate,

    #[error("Incorrect program owner for metadata account")]
    IncorrectMetadataProgramOwner,

    #[error("Incorrect program owner for mint account")]
    IncorrectMintProgramOwner,

    #[error("Incorrect program owner for migration state account")]
    IncorrectMigrationStateProgramOwner,

    #[error("Incorrect program owner for delegate record account")]
    IncorrectDelegateRecordProgramOwner,
}

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum DeserializationError {
    #[error("Metadata did not deserialize correctly")]
    InvalidMetadata = DESERIALIZATION_ERROR_START,

    #[error("Migration state did not deserialize correctly")]
    InvalidMigrationState,

    #[error("Empty migration state account")]
    EmptyMigrationState,

    #[error("Zeroed migration state account")]
    ZeroedMigrationState,

    #[error("Program signer did not deserialize correctly")]
    InvalidProgramSigner,

    #[error("Empty program signer account")]
    EmptyProgramSigner,
}

// General Error Impls
impl PrintProgramError for GeneralError {
    fn print<E>(&self) {
        msg!(
            "{} {}: {}",
            <crate::errors::GeneralError as DecodeError<E>>::type_of(),
            *self as u32,
            &self.to_string()
        );
    }
}

impl From<GeneralError> for ProgramError {
    fn from(e: GeneralError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for GeneralError {
    fn type_of() -> &'static str {
        "General Error"
    }
}

// Migration Error Impls
impl PrintProgramError for MigrationError {
    fn print<E>(&self) {
        msg!(
            "{} {}: {}",
            <crate::errors::MigrationError as DecodeError<E>>::type_of(),
            *self as u32,
            &self.to_string()
        );
    }
}

impl From<MigrationError> for ProgramError {
    fn from(e: MigrationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MigrationError {
    fn type_of() -> &'static str {
        "Migration Error"
    }
}

// Validation Error Impls
impl PrintProgramError for ValidationError {
    fn print<E>(&self) {
        msg!(
            "{} {}: {}",
            <crate::errors::ValidationError as DecodeError<E>>::type_of(),
            *self as u32,
            &self.to_string()
        );
    }
}

impl From<ValidationError> for ProgramError {
    fn from(e: ValidationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ValidationError {
    fn type_of() -> &'static str {
        "Validation Error"
    }
}

// Deserialization Error Impls
impl PrintProgramError for DeserializationError {
    fn print<E>(&self) {
        msg!(
            "{} {}: {}",
            <crate::errors::DeserializationError as DecodeError<E>>::type_of(),
            *self as u32,
            &self.to_string()
        );
    }
}

impl From<DeserializationError> for ProgramError {
    fn from(e: DeserializationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for DeserializationError {
    fn type_of() -> &'static str {
        "Deserialization Error"
    }
}
