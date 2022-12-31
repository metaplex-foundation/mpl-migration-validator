use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey, pubkey::Pubkey,
};

use crate::error::MigrationError;

pub(crate) const MIGRATION_WAIT_PERIOD: i64 = 60 * 60 * 24 * 14; // 14 days
pub(crate) const SPL_TOKEN_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
// Seeds: [b"migration", collection_mint.as_ref()]
pub struct MigrationState {
    pub collection_authority: Pubkey,
    pub collection_mint: Pubkey,
    pub rule_set: Pubkey,
    pub collection_delegate: Pubkey,
    pub start_time: i64,
    pub _type: Type,
    pub migration_size: u32,
    pub items_migrated: u32,
    pub in_progress: bool,
    pub is_eligible: bool,
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
}

#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug)]
pub enum Type {
    Timed,
    Vote,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct ProgramSigner {
    pub bump: u8,
}

impl ProgramSigner {
    pub fn address() -> Pubkey {
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
