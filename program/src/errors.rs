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
    #[error("")]
    Overflow,

    // 1, 0x1
    #[error("")]
    InvalidInstruction,

    // 2, 0x2
    #[error("")]
    NoRuleSet,

    // 3, 0x3
    #[error("")]
    FeatureDisabled,

    // 4, 0x4
    #[error("")]
    InvalidUnlockMethod,

    // Migration Errors

    // 5, 0x5
    #[error("")]
    MigrationInProgress,

    // 6, 0x6
    #[error("")]
    MigrationAlreadyCompleted,

    // 7, 0x7
    #[error("")]
    AlreadyInitialized,

    // 8, 0x8
    #[error("")]
    MigrationLocked,

    // 9, 0x9
    #[error("")]
    ImmutableMetadata,

    // 10, 0xA
    #[error("")]
    IncorrectFreezeAuthority,

    // 11, 0xB
    #[error("")]
    IncorrectTokenStandard,

    // 12, 0xC
    #[error("")]
    ImmutableProgramOwner,

    // Validation Errors

    // 13, 0xD
    #[error("")]
    MetadataMintMistmatch,

    // 14, 0xE
    #[error("")]
    TokenMintMismatch,

    // 15 0xF
    #[error("")]
    CollectionMintMismatch,

    // 16 0x10
    #[error("")]
    InvalidAuthority,

    // 17 0x11
    #[error("")]
    CollectionNotFound,

    // 18 0x12
    #[error("")]
    NotCollectionMember,

    // 19 0x13
    #[error("")]
    InvalidTokenStandard,

    // 20 0x14
    #[error("")]
    MissingTokenStandard,

    // 21 0x15
    #[error("")]
    InvalidMetadataDerivation,

    // 22 0x16
    #[error("")]
    InvalidEditionDerivation,

    // 23 0x17
    #[error("")]
    InvalidMigrationStateDerivation,

    // 24 0x18
    #[error("")]
    InvalidSignerDerivation,

    // 25 0x19
    #[error("")]
    InvalidDelegateRecordDerivation,

    // 26 0x1A
    #[error("")]
    InvalidDelegate,

    // 27 0x1B
    #[error("")]
    IncorrectMetadataProgramOwner,

    // 28 0x1C
    #[error("")]
    IncorrectMintProgramOwner,

    // 29 0x1D
    #[error("")]
    IncorrectMigrationStateProgramOwner,

    // 30 0x1E
    #[error("")]
    IncorrectDelegateRecordProgramOwner,

    // 31 0x1F
    #[error("")]
    TokenOwnerMismatch,

    // 32 0x20
    #[error("")]
    IncorrectTokenOwnerProgramOwner,

    // 33 0x21
    #[error("")]
    IncorrectTokenOwnerProgramBuffer,

    // Deserialization Errors

    // 34 0x22
    #[error("")]
    InvalidMetadata,

    // 35 0x23
    #[error("")]
    InvalidMigrationState,

    // 36 0x24
    #[error("")]
    EmptyMigrationState,

    // 37 0x25
    #[error("")]
    ZeroedMigrationState,

    // 38 0x26
    #[error("")]
    InvalidProgramSigner,

    // 39 0x27
    #[error("")]
    EmptyProgramSigner,

    // 40 0x28
    #[error("")]
    InvalidUpgradeableLoaderState,

    // 41 0x29
    #[error("")]
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
