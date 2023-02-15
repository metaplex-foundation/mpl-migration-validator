use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MigrationError {
    // 0, 0x0
    #[error("Overflow error")]
    Overflow,

    // 1, 0x1
    #[error("Failed to build Migrate instruction")]
    InvalidInstruction,

    // 2, 0x2
    #[error("No rule set provided")]
    NoRuleSet,

    // 3, 0x3
    #[error("This feature is currently disabled")]
    FeatureDisabled,

    // 4, 0x4
    #[error("Invalid unlock method")]
    InvalidUnlockMethod,

    // Migration Errors

    // 5, 0x5
    #[error("Cannot perform this action while migration is in progress")]
    MigrationInProgress,

    // 6, 0x6
    #[error("Cannot be closed after migration has completed")]
    MigrationAlreadyCompleted,

    // 7, 0x7
    #[error("Program signer is already initialized")]
    AlreadyInitialized,

    // 8, 0x8
    #[error("Migration state account is locked")]
    MigrationLocked,

    // 9, 0x9
    #[error("Immutable metadata cannot be migrated")]
    ImmutableMetadata,

    // 10, 0xA
    #[error("Incorrect freeze authority")]
    IncorrectFreezeAuthority,

    // 11, 0xB
    #[error("Incorrect token standard: must be NonFungible")]
    IncorrectTokenStandard,

    // 12, 0xC
    #[error("Cannot migrate an item owned by an immutable program")]
    ImmutableProgramOwner,

    // Validation Errors

    // 13, 0xD
    #[error("Metadata does not match mint account")]
    MetadataMintMistmatch,

    // 14, 0xE
    #[error("Token does not match the mint account")]
    TokenMintMismatch,

    // 15 0xF
    #[error("Collection mint does not match stored value")]
    CollectionMintMismatch,

    // 16 0x10
    #[error("Authority does not match the authority on the account")]
    InvalidAuthority,

    // 17 0x11
    #[error("No collection found on item")]
    CollectionNotFound,

    // 18 0x12
    #[error("Item is not a verified member of the collection")]
    NotCollectionMember,

    // 19 0x13
    #[error("Invalid token standard")]
    InvalidTokenStandard,

    // 20 0x14
    #[error("Missing token standard")]
    MissingTokenStandard,

    // 21 0x15
    #[error("The metadata derivation does not match the mint account")]
    InvalidMetadataDerivation,

    // 22 0x16
    #[error("The edition derivation does not match the mint account")]
    InvalidEditionDerivation,

    // 23 0x17
    #[error("Migration state account derivation is in correct")]
    InvalidMigrationStateDerivation,

    // 24 0x18
    #[error("Program signer account derivation is incorrect")]
    InvalidSignerDerivation,

    // 25 0x19
    #[error("Invalid delegate record derivation")]
    InvalidDelegateRecordDerivation,

    // 26 0x1A
    #[error("Invalid delegate")]
    InvalidDelegate,

    // 27 0x1B
    #[error("Incorrect program owner for metadata account")]
    IncorrectMetadataProgramOwner,

    // 28 0x1C
    #[error("Incorrect program owner for mint account")]
    IncorrectMintProgramOwner,

    // 29 0x1D
    #[error("Incorrect program owner for migration state account")]
    IncorrectMigrationStateProgramOwner,

    // 30 0x1E
    #[error("Incorrect program owner for delegate record account")]
    IncorrectDelegateRecordProgramOwner,

    // 31 0x1F
    #[error("Incorrect owner for SPL token account")]
    TokenOwnerMismatch,

    // 32 0x20
    #[error("Incorrect program owner for token owner account")]
    IncorrectTokenOwnerProgramOwner,

    // 33 0x21
    #[error("Incorrect program owner for token owner account buffer")]
    IncorrectTokenOwnerProgramBuffer,

    // Deserialization Errors

    // 34 0x22
    #[error("Metadata did not deserialize correctly")]
    InvalidMetadata,

    // 35 0x23
    #[error("Migration state did not deserialize correctly")]
    InvalidMigrationState,

    // 36 0x24
    #[error("Empty migration state account")]
    EmptyMigrationState,

    // 37 0x25
    #[error("Zeroed migration state account")]
    ZeroedMigrationState,

    // 38 0x26
    #[error("Program signer did not deserialize correctly")]
    InvalidProgramSigner,

    // 39 0x27
    #[error("Empty program signer account")]
    EmptyProgramSigner,

    // 40 0x28
    #[error("Failed to deserialize UpgradeableLoaderState")]
    InvalidUpgradeableLoaderState,

    // 41 0x29
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
