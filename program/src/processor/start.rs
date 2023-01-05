use mpl_token_metadata::state::CollectionAuthorityRecord;

use super::*;

pub fn start_migration(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let payer_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let collection_mint_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let delegate_info = next_account_info(account_info_iter)?;
    let delegate_record_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;
    let spl_token_program_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let _token_metadata_program_info = next_account_info(account_info_iter)?;

    // Authority is a signer
    assert_signer(authority_info)?;

    // The migration state account must must match the correct derivation
    let _bump = assert_derivation(
        program_id,
        migration_state_info,
        &[b"migration", collection_mint_info.key.as_ref()],
        MigrationError::InvalidStateDerivation,
    )?;
    // let state_seeds = &[b"migration", collection_mint_info.key.as_ref(), &[bump]];

    // Deserialize the migration state
    let mut migration_state = MigrationState::from_account_info(migration_state_info)?;

    if collection_mint_info.key != &migration_state.collection_info.mint {
        return Err(MigrationError::CollectionMintMismatch.into());
    }

    let program_signer = ProgramSigner::pubkey();

    // The delegate record must match the correct derivation
    // with the mint from the migration state account and the
    // program signer as the delegate.
    assert_derivation(
        &mpl_token_metadata::ID,
        delegate_record_info,
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            migration_state.collection_info.mint.as_ref(),
            mpl_token_metadata::pda::COLLECTION_AUTHORITY.as_bytes(),
            program_signer.as_ref(),
        ],
        MigrationError::InvalidDelegateRecordDerivation,
    )?;

    // If the delegate record is unitialized, then we CPI into
    // the token metadata program to initialize it.
    if delegate_record_info.data_is_empty() {
        msg!("Initializing delegate record");
        let instruction = mpl_token_metadata::instruction::approve_collection_authority(
            mpl_token_metadata::ID,
            *delegate_record_info.key,
            program_signer,
            *authority_info.key,
            *payer_info.key,
            *collection_metadata_info.key,
            *collection_mint_info.key,
        );
        let account_infos = vec![
            delegate_record_info.clone(),
            authority_info.clone(),
            payer_info.clone(),
            delegate_info.clone(),
            collection_metadata_info.clone(),
            collection_mint_info.clone(),
            spl_token_program_info.clone(),
            system_program_info.clone(),
        ];

        invoke_signed(&instruction, &account_infos, &[]).unwrap();
    }

    if spl_token_program_info.key != &SPL_TOKEN_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    if system_program_info.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let authority_record = CollectionAuthorityRecord::from_account_info(delegate_record_info)?;

    if authority_record.update_authority != Some(*authority_info.key) {
        return Err(MigrationError::InvalidAuthority.into());
    }

    // Migration must be unlocked
    if migration_state.status.is_locked {
        return Err(MigrationError::MigrationLocked.into());
    }

    migration_state.collection_info.delegate = *delegate_info.key;
    migration_state.status.in_progress = true;
    migration_state.save(migration_state_info)?;

    Ok(())
}
