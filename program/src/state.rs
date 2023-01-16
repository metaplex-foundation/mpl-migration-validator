use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_memory::sol_memcpy, pubkey,
    pubkey::Pubkey,
};

#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

use crate::errors::{DeserializationError, GeneralError};

pub(crate) const MIGRATION_WAIT_PERIOD: i64 = 60 * 60 * 24 * 14; // 14 days
pub(crate) const SPL_TOKEN_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
// Seeds: [b"migration", collection_mint.as_ref()]
pub struct MigrationState {
    pub collection_info: CollectionInfo,
    pub unlock_method: UnlockMethod,
    pub status: MigrationStatus,
}

impl MigrationState {
    pub fn from_account_info(a: &AccountInfo) -> Result<Self, ProgramError> {
        let data = a.try_borrow_data()?;

        if data.is_empty() {
            return Err(DeserializationError::EmptyMigrationState.into());
        }

        if data.is_zeroed() {
            return Err(DeserializationError::ZeroedMigrationState.into());
        }

        let ua = <Self as BorshDeserialize>::deserialize(&mut data.as_ref())
            .map_err(|_| DeserializationError::InvalidMigrationState)?;

        Ok(ua)
    }

    pub fn save(&self, a: &AccountInfo) -> Result<(), ProgramError> {
        let serialized_data = self.try_to_vec()?;
        let data_len = serialized_data.len();

        sol_memcpy(
            &mut a.data.borrow_mut(),
            serialized_data.as_slice(),
            data_len,
        );

        Ok(())
    }
}

impl Default for MigrationState {
    fn default() -> Self {
        Self {
            collection_info: CollectionInfo::default(),
            unlock_method: UnlockMethod::Timed,
            status: MigrationStatus::default(),
        }
    }
}

trait Zeroed {
    fn is_zeroed(&self) -> bool;
}

impl Zeroed for [u8] {
    fn is_zeroed(&self) -> bool {
        self.iter().all(|&x| x == 0)
    }
}

#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, Default, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct CollectionInfo {
    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub authority: Pubkey,

    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub mint: Pubkey,

    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub rule_set: Pubkey,

    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub delegate_record: Pubkey,

    pub size: u32,
}

#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, Default, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MigrationStatus {
    pub unlock_time: i64,
    pub is_locked: bool,
    pub in_progress: bool,
    pub items_migrated: u32,
}

#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug)]
pub enum UnlockMethod {
    Timed,
    Vote,
}

impl FromStr for UnlockMethod {
    type Err = GeneralError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "timed" => Ok(UnlockMethod::Timed),
            "vote" => Ok(UnlockMethod::Vote),
            _ => Err(GeneralError::InvalidUnlockMethod),
        }
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct ProgramSigner {
    pub bump: u8,
}

impl ProgramSigner {
    pub fn pubkey() -> Pubkey {
        pubkey!("4fDQAj27ahBfXw3ZQumg5gJrMRUCzPUW6RxrRPFMC8Av")
    }
    pub fn from_account_info(a: &AccountInfo) -> Result<Self, ProgramError> {
        let data = a.try_borrow_data()?;

        if data.is_empty() {
            return Err(DeserializationError::EmptyProgramSigner.into());
        }

        let ua = Self::deserialize(&mut data.as_ref())
            .map_err(|_| DeserializationError::InvalidProgramSigner)?;

        Ok(ua)
    }
}
