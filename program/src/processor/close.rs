use crate::utils::close_program_account;

use super::*;

pub fn close_migration_state(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Migration Validator: Close");
    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;
    let _system_program_info = next_account_info(account_info_iter)?;

    // Validate Accounts
    assert_signer(authority_info)?;

    // Paranoia.
    assert_owned_by(
        migration_state_info,
        program_id,
        ValidationError::IncorrectProgramOwner,
    )?;

    // Deserialize the migration state
    let buffer = migration_state_info.try_borrow_data()?;
    let migration_state = MigrationState::deserialize(&mut buffer.as_ref())
        .map_err(|_| DeserializationError::InvalidMigrationState)?;

    // Idc about compute, check this anyway.
    assert_derivation(
        program_id,
        migration_state_info,
        &[b"migration", migration_state.collection_info.mint.as_ref()],
        ValidationError::InvalidMigrationStateDerivation,
    )?;

    // Ensure the authority matches
    if migration_state.collection_info.authority != *authority_info.key {
        return Err(ValidationError::InvalidAuthority.into());
    }

    // Ensure the migration isn't in progress
    if migration_state.status.in_progress {
        return Err(MigrationError::MigrationInProgress.into());
    }

    drop(buffer);
    close_program_account(migration_state_info, authority_info)?;

    Ok(())
}
