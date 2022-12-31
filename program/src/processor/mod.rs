use crate::{
    error::MigrationError,
    instruction::{InitializeArgs, MigrationInstruction, UpdateArgs},
    state::{MigrationState, ProgramSigner, Type, MIGRATION_WAIT_PERIOD, SPL_TOKEN_ID},
};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{
    instruction::{builders::MigrateBuilder, InstructionBuilder, MigrateArgs},
    state::{Metadata, MigrationType, TokenMetadataAccount},
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
    pubkey::Pubkey,
    sysvar::Sysvar,
};

mod close;
mod initialize;
mod migrate;
mod update;

use close::close_migration_state;
use initialize::initialize_migration;
use migrate::migrate_item;
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
                migrate_item(program_id, accounts)
            }
            MigrationInstruction::Migrate => {
                // handle instruction
                Ok(())
            }
            MigrationInstruction::InitSigner => init_signer(program_id, accounts),
        }
    }
}

fn init_signer(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_info = next_account_info(account_info_iter)?;
    let program_signer_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    let bump = assert_derivation(
        program_id,
        program_signer_info,
        &[b"signer"],
        MigrationError::InvalidSignerDerivation,
    )?;

    if system_program_info.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Already initialized
    if !program_signer_info.data.borrow().is_empty() {
        return Err(MigrationError::AlreadyInitialized.into());
    }

    let signer = ProgramSigner { bump };

    let serialized_data = signer.try_to_vec()?;
    let data_len = serialized_data.len();

    mpl_utils::create_or_allocate_account_raw(
        *program_id,
        program_signer_info,
        system_program_info,
        payer_info,
        data_len,
        &[],
    )?;

    msg!("writing state");
    sol_memcpy(
        &mut program_signer_info.data.borrow_mut(),
        serialized_data.as_slice(),
        data_len,
    );

    Ok(())
}
