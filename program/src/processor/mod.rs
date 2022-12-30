use crate::{
    error::MigrateError,
    instruction::{InitializeArgs, MigrateInstruction},
    state::{MigrationState, MIGRATION_WAIT_PERIOD},
};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use mpl_utils::{assert_derivation, assert_signer};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

mod close;
mod initialize;

use close::close_migration_state;
use initialize::initialize_migration;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: MigrateInstruction = MigrateInstruction::try_from_slice(instruction_data)?;

        match instruction {
            MigrateInstruction::Initialize(args) => {
                // handle instruction
                initialize_migration(program_id, accounts, args)
            }
            MigrateInstruction::Start => {
                // handle instruction
                Ok(())
            }
            MigrateInstruction::Close => {
                // handle instruction
                close_migration_state(program_id, accounts)
            }
            MigrateInstruction::Migrate => {
                // handle instruction
                Ok(())
            }
        }
    }
}
