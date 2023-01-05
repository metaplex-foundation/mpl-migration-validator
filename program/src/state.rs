use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_memory::sol_memcpy, pubkey,
    pubkey::Pubkey,
};

use crate::error::MigrationError;

pub(crate) const MIGRATION_WAIT_PERIOD: i64 = 60 * 60 * 24 * 14; // 14 days
pub(crate) const SPL_TOKEN_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

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
            return Err(MigrationError::InvalidStateDerivation.into());
        }

        let ua = Self::deserialize(&mut data.as_ref())
            .map_err(|_| MigrationError::InvalidStateDerivation)?;

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

impl Default for CollectionInfo {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            mint: Pubkey::default(),
            rule_set: Pubkey::default(),
            delegate: Pubkey::default(),
            size: 0,
        }
    }
}

impl Default for MigrationStatus {
    fn default() -> Self {
        Self {
            unlock_time: 0,
            is_locked: true,
            in_progress: false,
            items_migrated: 0,
        }
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct CollectionInfo {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub rule_set: Pubkey,
    pub delegate: Pubkey,
    pub size: u32,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MigrationStatus {
    pub unlock_time: i64,
    pub is_locked: bool,
    pub in_progress: bool,
    pub items_migrated: u32,
}

#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug)]
pub enum UnlockMethod {
    Timed,
    Vote,
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
            return Err(MigrationError::InvalidStateDerivation.into());
        }

        let ua = Self::deserialize(&mut data.as_ref())
            .map_err(|_| MigrationError::InvalidStateDerivation)?;

        Ok(ua)
    }
}
