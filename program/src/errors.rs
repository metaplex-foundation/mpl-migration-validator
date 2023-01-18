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
    MigrationAlreadyCompleted = 1 + MIGRATION_ERROR_START,

    #[error("Program signer is already initialized")]
    AlreadyInitialized = 2 + MIGRATION_ERROR_START,

    #[error("Migration state account is locked")]
    MigrationLocked = 3 + MIGRATION_ERROR_START,

    #[error("Immutable metadata cannot be migrated")]
    ImmutableMetadata = 4 + MIGRATION_ERROR_START,

    #[error("Incorrect freeze authority")]
    IncorrectFreezeAuthority = 5 + MIGRATION_ERROR_START,
}

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum ValidationError {
    #[error("Metadata does not match mint account")]
    MetadataMintMistmatch = VALIDATOR_ERROR_START,

    #[error("Token does not match the mint account")]
    TokenMintMismatch = 1 + VALIDATOR_ERROR_START,

    #[error("Collection mint does not match stored value")]
    CollectionMintMismatch = 2 + VALIDATOR_ERROR_START,

    #[error("Authority does not match the authority on the account")]
    InvalidAuthority = 3 + VALIDATOR_ERROR_START,

    #[error("No collection found on item")]
    CollectionNotFound = 4 + VALIDATOR_ERROR_START,

    #[error("Item is not a verified member of the collection")]
    NotCollectionMember = 5 + VALIDATOR_ERROR_START,

    #[error("Invalid token standard")]
    InvalidTokenStandard = 6 + VALIDATOR_ERROR_START,

    #[error("Missing token standard")]
    MissingTokenStandard = 7 + VALIDATOR_ERROR_START,

    #[error("The metadata derivation does not match the mint account")]
    InvalidMetadataDerivation = 8 + VALIDATOR_ERROR_START,

    #[error("The edition derivation does not match the mint account")]
    InvalidEditionDerivation = 9 + VALIDATOR_ERROR_START,

    #[error("Migration state account derivation is in correct")]
    InvalidMigrationStateDerivation = 10 + VALIDATOR_ERROR_START,

    #[error("Program signer account derivation is incorrect")]
    InvalidSignerDerivation = 11 + VALIDATOR_ERROR_START,

    #[error("Invalid delegate record derivation")]
    InvalidDelegateRecordDerivation = 12 + VALIDATOR_ERROR_START,

    #[error("Invalid delegate")]
    InvalidDelegate = 13 + VALIDATOR_ERROR_START,

    #[error("Incorrect program owner for metadata account")]
    IncorrectMetadataProgramOwner = 14 + VALIDATOR_ERROR_START,

    #[error("Incorrect program owner for mint account")]
    IncorrectMintProgramOwner = 15 + VALIDATOR_ERROR_START,

    #[error("Incorrect program owner for migration state account")]
    IncorrectMigrationStateProgramOwner = 16 + VALIDATOR_ERROR_START,

    #[error("Incorrect program owner for delegate record account")]
    IncorrectDelegateRecordProgramOwner = 17 + VALIDATOR_ERROR_START,
}

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum DeserializationError {
    #[error("Metadata did not deserialize correctly")]
    InvalidMetadata = DESERIALIZATION_ERROR_START,

    #[error("Migration state did not deserialize correctly")]
    InvalidMigrationState = 1 + DESERIALIZATION_ERROR_START,

    #[error("Empty migration state account")]
    EmptyMigrationState = 2 + DESERIALIZATION_ERROR_START,

    #[error("Zeroed migration state account")]
    ZeroedMigrationState = 3 + DESERIALIZATION_ERROR_START,

    #[error("Program signer did not deserialize correctly")]
    InvalidProgramSigner = 4 + DESERIALIZATION_ERROR_START,

    #[error("Empty program signer account")]
    EmptyProgramSigner = 5 + DESERIALIZATION_ERROR_START,
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
