use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

pub(crate) const MIGRATION_WAIT_PERIOD: i64 = 60 * 60 * 24 * 14; // 14 days

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
// Seeds: [b"migration", collection_mint.as_ref()]
pub struct MigrationState {
    pub collection_authority: Pubkey,
    pub collection_mint: Pubkey,
    pub rule_set: Pubkey,
    pub collection_delegate: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub migration_type: MigrationType,
    pub migration_size: u32,
    pub in_progress: bool,
    pub is_eligible: bool,
}

impl MigrationState {
    pub fn collection(&self) -> Pubkey {
        self.collection_mint
    }

    pub fn rule_set(&self) -> Pubkey {
        self.rule_set
    }

    pub fn collection_delegate(&self) -> Pubkey {
        self.collection_delegate
    }

    pub fn start_time(&self) -> i64 {
        self.start_time
    }

    pub fn end_time(&self) -> i64 {
        self.end_time
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug)]
pub enum MigrationType {
    WaitPeriod,
    Vote,
}
