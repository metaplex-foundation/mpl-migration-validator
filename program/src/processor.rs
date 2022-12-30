use crate::{
    error::MigrateError,
    instruction::{InitializeArgs, MigrateInstruction},
    state::{MigrationState, MIGRATION_STATE_SIZE, MIGRATION_WAIT_PERIOD},
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
            MigrateInstruction::Cancel => {
                // handle instruction
                Ok(())
            }
            MigrateInstruction::Migrate => {
                // handle instruction
                Ok(())
            }
        }
    }
}

pub fn initialize_migration(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitializeArgs,
) -> ProgramResult {
    msg!("Initiate Migration");
    let InitializeArgs {
        rule_set,
        migration_type,
    } = args;

    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let payer_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let collection_mint_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Validate Accounts

    // Both accounts must be signers, but can be the same account
    // if collection authority is paying.
    assert_signer(payer_info)?;
    assert_signer(authority_info)?;

    // Ensure that these accounts all belong together
    // * Metadata must be derived from the mint address
    // * Authority must be the update_authority on the metadata struct

    // Properly derived Metadata account
    assert_derivation(
        &mpl_token_metadata::ID,
        collection_metadata_info,
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            collection_mint_info.key.as_ref(),
        ],
        MigrateError::MetadataMintMistmatch,
    )?;

    // This ensures the account isn't empty as the deserialization fails if the account doesn't have the correct size.
    let metadata = Metadata::from_account_info(collection_metadata_info)
        .map_err(|_| MigrateError::InvalidMetadata)?;

    // Ensure that the authority is the update authority on the metadata
    if metadata.update_authority != *authority_info.key {
        return Err(MigrateError::InvalidAuthority.into());
    }

    // The migrate state account must must match the correct derivation
    let bump = assert_derivation(
        program_id,
        migration_state_info,
        &[b"migration", collection_mint_info.key.as_ref()],
        MigrateError::InvalidStateAccount,
    )?;
    let state_seeds = &[b"migration", collection_mint_info.key.as_ref(), &[bump]];

    if system_program_info.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("accounts validated");

    let start_time = Clock::get()?.unix_timestamp;
    let end_time = start_time + MIGRATION_WAIT_PERIOD;

    let migrate_state = MigrationState {
        collection_mint: *collection_mint_info.key,
        rule_set: rule_set.unwrap_or_default(),
        start_time,
        end_time,
        migration_type,
        migration_eligible: false,
        collection_delegate: Pubkey::default(),
    };

    mpl_utils::create_or_allocate_account_raw(
        *program_id,
        migration_state_info,
        system_program_info,
        payer_info,
        MIGRATION_STATE_SIZE,
        state_seeds,
    )?;

    msg!("serializing state");
    migrate_state.serialize(&mut *migration_state_info.data.borrow_mut())?;

    Ok(())
}
