use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

pub(crate) const MIGRATION_WAIT_PERIOD: i64 = 60 * 60 * 24 * 14; // 14 days

pub const MIGRATION_STATE_SIZE: usize = 32 + 32 + 8 + 8 + 1;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
// Seeds: [b"migration", collection_mint.as_ref()]
pub struct MigrationState {
    collection_mint: Pubkey,
    rule_set: Pubkey,
    start_time: i64,
    end_time: i64,
    migration_eligible: bool,
}

impl MigrationState {
    pub fn new(collection_mint: Pubkey, rule_set: Pubkey, start_time: i64, end_time: i64) -> Self {
        Self {
            collection_mint,
            rule_set,
            start_time,
            end_time,
            migration_eligible: false,
        }
    }

    pub fn collection(&self) -> Pubkey {
        self.collection_mint
    }

    pub fn rule_set(&self) -> Pubkey {
        self.rule_set
    }

    pub fn start_time(&self) -> i64 {
        self.start_time
    }

    pub fn end_time(&self) -> i64 {
        self.end_time
    }
}
