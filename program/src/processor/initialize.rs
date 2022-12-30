use super::*;

use solana_program::program_memory::sol_memcpy;

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
        MigrateError::InvalidStateDerivation,
    )?;
    let state_seeds = &[b"migration", collection_mint_info.key.as_ref(), &[bump]];

    if system_program_info.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("accounts validated");

    let start_time = Clock::get()?.unix_timestamp;
    let end_time = start_time + MIGRATION_WAIT_PERIOD;

    let migration_state = MigrationState {
        collection_authority: *authority_info.key,
        collection_mint: *collection_mint_info.key,
        rule_set: rule_set.unwrap_or_default(),
        start_time,
        end_time,
        migration_type,
        migration_size: 0,
        is_eligible: false,
        in_progress: false,
        collection_delegate: Pubkey::default(),
    };

    let serialized_data = migration_state.try_to_vec()?;
    let data_len = serialized_data.len();

    mpl_utils::create_or_allocate_account_raw(
        *program_id,
        migration_state_info,
        system_program_info,
        payer_info,
        data_len,
        state_seeds,
    )?;

    msg!("writing state");
    sol_memcpy(
        &mut *migration_state_info.data.borrow_mut(),
        serialized_data.as_slice(),
        data_len,
    );

    Ok(())
}
