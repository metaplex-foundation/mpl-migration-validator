use crate::{
    errors::{MigrationError, ValidationError},
    instruction::{InitializeArgs, MigrationInstruction, UpdateArgs},
    state::{MigrationState, ProgramSigner, UnlockMethod, MIGRATION_WAIT_PERIOD, SPL_TOKEN_ID},
};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{
    instruction::MigrateArgs,
    state::{Metadata, MigrationType, TokenMetadataAccount, EDITION, PREFIX},
};
use mpl_utils::{assert_derivation, assert_owned_by, assert_signer};
use solana_program::program::invoke_signed;
use solana_program::program_memory::sol_memcpy;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    sysvar::{self, Sysvar},
};

use spl_token::state::Account as TokenAccount;

mod close;
mod initialize;
mod migrate;
mod misc;
mod start;
mod update;
mod validators;

use crate::errors::*;
use close::close_migration_state;
use initialize::initialize_migration;
use migrate::migrate_item;
use misc::init_signer;
use start::start_migration;
use update::update_state;
use validators::*;

pub struct Processor;
impl Processor {
    pub fn process_instruction<'a>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
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
                start_migration(program_id, accounts)
            }
            MigrationInstruction::Migrate => {
                // handle instruction
                migrate_item(program_id, accounts)
            }
            MigrationInstruction::InitSigner => init_signer(program_id, accounts),
        }
    }
}
