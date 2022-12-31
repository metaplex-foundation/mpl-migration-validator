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
            .map_err(|_| MigrationError::InvalidStateDeserialization)?;

        // Ensure the authority matches
        if migration_state.collection_info.authority != *authority_info.key {
            return Err(MigrationError::InvalidAuthority.into());
        }

        // Ensure the migration isn't in progress
        if migration_state.status.in_progress {
            return Err(MigrationError::MigrationInProgress.into());
        }
    }

    mpl_utils::close_account_raw(authority_info, migration_state_info)?;

    Ok(())
}
