pub mod entrypoint;
pub mod errors;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

pub use solana_program;
use solana_program::{pubkey, pubkey::Pubkey};

solana_program::declare_id!("migrxZFChTqicHpNa1CAjPcF29Mui2JU2q4Ym7qQUTi");

pub const PROGRAM_SIGNER: Pubkey = pubkey!("4fDQAj27ahBfXw3ZQumg5gJrMRUCzPUW6RxrRPFMC8Av");
