use super::*;

pub fn update_state(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: UpdateArgs,
) -> ProgramResult {
    msg!("Update State");

    let UpdateArgs { rule_set } = args;

    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;

    // Use peekable to check if there are any more accounts and if it it's
    // owned by the expected voting/governance program.

    // Validate Accounts
    assert_signer(authority_info)?;

    assert_owned_by(
        migration_state_info,
        &crate::ID,
        ValidationError::IncorrectProgramOwner,
    )?;

    // Deserialize the migration state
    let mut migration_state = MigrationState::from_account_info(migration_state_info)?;

    // Ensure the authority matches
    incoming_collection_authority_matches_stored(authority_info, &migration_state)?;

    // Ensure the migration isn't in progress
    if migration_state.status.in_progress {
        return Err(MigrationError::MigrationInProgress.into());
    }

    // Check for state changes
    let mut state_change = false;

    // If given a rule_set, update the state.
    if let Some(rule_set) = rule_set {
        msg!("new rule set provided");
        msg!("rule set: {:?}", rule_set);
        migration_state.collection_info.rule_set = rule_set;
        state_change = true;
    }

    // Perform a time check to check eligibility for migration
    let now = Clock::get()?.unix_timestamp;
    msg!("now: {:?}", now);
    let wait_period_over = now >= migration_state.status.unlock_time;

    if migration_state.unlock_method == UnlockMethod::Timed && wait_period_over {
        migration_state.status.is_locked = false;
        state_change = true;
    }

    // If provided a spl governance account and the type is Vote
    // then perform checks.
    // Wait period must be over in addition to whatever voting checks
    // are required.
    // state_change = true;

    // write updated state if there was a change
    if state_change {
        msg!("Updating state");
        migration_state.save(migration_state_info)?;
    }

    Ok(())
}
