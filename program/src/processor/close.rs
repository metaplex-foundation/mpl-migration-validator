use super::*;

pub fn close_migration_state(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;

    // Validate Accounts
    assert_signer(authority_info)?;

    {
        // Deserialize the migration state
        let buffer = migration_state_info.try_borrow_data()?;
        let migration_state = MigrationState::deserialize(&mut buffer.as_ref())
            .map_err(|_| MigrateError::InvalidStateDeserialization)?;

        // Ensure the authority matches
        if migration_state.collection_authority != *authority_info.key {
            return Err(MigrateError::InvalidAuthority.into());
        }

        // Ensure the migration isn't in progress
        if migration_state.in_progress {
            return Err(MigrateError::MigrationInProgress.into());
        }
    }

    mpl_utils::close_account_raw(authority_info, migration_state_info)?;

    Ok(())
}
