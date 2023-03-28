use crate::{errors::MigrationError, instruction::MigrationInstruction, state::MigrationState};
use borsh::BorshDeserialize;
use mpl_utils::{assert_derivation, assert_owned_by, assert_signer};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

mod close;
mod misc;
mod update;
mod validator;

use close::close_migration_state;

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
            MigrationInstruction::Initialize(_) => {
                Err(MigrationError::MigrationWindowClosed.into())
            }
            MigrationInstruction::Update(_) => Err(MigrationError::MigrationWindowClosed.into()),
            MigrationInstruction::Close => close_migration_state(program_id, accounts),
            MigrationInstruction::Start => Err(MigrationError::MigrationWindowClosed.into()),
            MigrationInstruction::Migrate => Err(MigrationError::MigrationWindowClosed.into()),
            MigrationInstruction::InitSigner => Err(MigrationError::MigrationWindowClosed.into()),
        }
    }
}
