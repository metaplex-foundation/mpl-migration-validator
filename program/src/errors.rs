use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MigrationError {
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

    // Migration Errors
    #[error("Cannot perform this action while migration is in progress")]
    MigrationInProgress,

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

    #[error("Cannot migrate an item owned by an immutable program")]
    ImmutableProgramOwner,

    // Validation Errors
    #[error("Metadata does not match mint account")]
    MetadataMintMistmatch,

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

    #[error("Incorrect owner for SPL token account")]
    TokenOwnerMismatch,

    #[error("Incorrect program owner for token owner account")]
    IncorrectTokenOwnerProgramOwner,

    #[error("Incorrect program owner for token owner account buffer")]
    IncorrectTokenOwnerProgramBuffer,

    // Deserialization Errors
    #[error("Metadata did not deserialize correctly")]
    InvalidMetadata,

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

    #[error("Failed to deserialize UpgradeableLoaderState")]
    InvalidUpgradeableLoaderState,

    #[error("Authorization rules does not match the rule set stored on the state")]
    InvalidRuleSet,
}

// Migration Error Impls
impl PrintProgramError for MigrationError {
    fn print<E>(&self) {
        msg!("Error {}: {}", *self as u32, &self.to_string());
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
