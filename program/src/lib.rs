pub mod entrypoint;
pub mod errors;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

pub use solana_program;
use solana_program::{pubkey, pubkey::Pubkey};

solana_program::declare_id!("migrxZFChTqicHpNa1CAjPcF29Mui2JU2q4Ym7qQUTi");

const MPL_TOKEN_AUTH_RULES_ID: Pubkey = pubkey!("auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg");
