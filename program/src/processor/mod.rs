use crate::state::MigrationType;
use crate::{
    error::MigrationError,
    instruction::{InitializeArgs, MigrationInstruction, UpdateArgs},
    state::{MigrationState, MIGRATION_WAIT_PERIOD},
};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use mpl_utils::{assert_derivation, assert_owned_by, assert_signer};
use solana_program::program_memory::sol_memcpy;
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
mod update;

use close::close_migration_state;
use initialize::initialize_migration;
use update::update_state;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: MigrationInstruction =
            MigrationInstruction::try_from_slice(instruction_data)?;

        match instruction {
            MigrationInstruction::Initialize(args) => {
                // handle instruction
                initialize_migration(program_id, accounts, args)
            }
            MigrationInstruction::Update(args) => {
                // handle instruction
                update_state(program_id, accounts, args)
            }
            MigrationInstruction::Close => {
                // handle instruction
                close_migration_state(program_id, accounts)
            }
            MigrationInstruction::Start => {
                // handle instruction
                Ok(())
            }
            MigrationInstruction::Migrate => {
                // handle instruction
                Ok(())
            }
        }
    }
}
