use super::*;

pub fn start_migration(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    return Err(MigrationError::FeatureDisabled.into());
    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let collection_mint_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Validate Accounts

    assert_signer(authority_info)?;

    Ok(())
}
