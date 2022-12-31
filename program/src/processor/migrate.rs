use crate::utils::assert_valid_delegate;

use super::*;

pub fn migrate_item(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Migrate Item");

    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let payer_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let token_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let delegate_record_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;
    let program_signer_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let sysvar_instructions_info = next_account_info(account_info_iter)?;
    let spl_token_program_info = next_account_info(account_info_iter)?;
    let token_metadata_program_info = next_account_info(account_info_iter)?;

    // Validate Accounts

    // Signers
    assert_signer(payer_info)?;

    // Program ownership
    assert_owned_by(
        delegate_record_info,
        &mpl_token_metadata::ID,
        MigrationError::IncorrectProgramOwner,
    )?;

    assert_owned_by(
        mint_info,
        &SPL_TOKEN_ID,
        MigrationError::IncorrectProgramOwner,
    )?;
    assert_owned_by(
        metadata_info,
        &mpl_token_metadata::ID,
        MigrationError::IncorrectProgramOwner,
    )?;
    assert_owned_by(
        migration_state_info,
        program_id,
        MigrationError::IncorrectProgramOwner,
    )?;

    // Programs
    if token_metadata_program_info.key != &mpl_token_metadata::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    if system_program_info.key != &solana_program::system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // This ensures the account isn't empty as the deserialization fails if
    // the account doesn't have the correct size.
    let metadata =
        Metadata::from_account_info(metadata_info).map_err(|_| MigrationError::InvalidMetadata)?;

    let collection_metadata = Metadata::from_account_info(collection_metadata_info)
        .map_err(|_| MigrationError::InvalidMetadata)?;

    // Deserialize the migration state
    let mut migration_state = MigrationState::from_account_info(migration_state_info)?;

    // This also checks that the data is not empty to ensure this is initalized.
    let program_signer = ProgramSigner::from_account_info(program_signer_info)?;
    let signers_seeds = &[b"signer", crate::ID.as_ref(), &[program_signer.bump]];

    // Validate that the delegate is the program signer for the correct
    // mint and update authority.
    assert_valid_delegate(
        &program_signer_info.key,
        delegate_record_info,
        &collection_metadata,
        &migration_state,
    )?;

    if migration_state.rule_set == Pubkey::default() {
        return Err(MigrationError::NoRuleSet.into());
    }

    let args = MigrateArgs::V1 {
        migration_type: MigrationType::ProgrammableV1,
        rule_set: Some(migration_state.rule_set),
    };

    let mut builder = MigrateBuilder::new();
    let instruction = builder
        .mint(*mint_info.key)
        .metadata(*metadata_info.key)
        .edition(*edition_info.key)
        .token(*token_info.key)
        .payer(*payer_info.key)
        .collection_metadata(*collection_metadata_info.key)
        .authority(*program_signer_info.key)
        // .delegate_record(*delegate_record_info.key)
        .build(args)
        .map_err(|_| MigrationError::InvalidInstruction)?
        .instruction();

    let account_infos = [
        metadata_info.clone(),
        edition_info.clone(),
        token_info.clone(),
        mint_info.clone(),
        program_signer_info.clone(),
        // delegate_record_info.clone(),
        collection_metadata_info.clone(),
        system_program_info.clone(),
        sysvar_instructions_info.clone(),
        spl_token_program_info.clone(),
    ];

    invoke_signed(&instruction, &account_infos, &[signers_seeds]).unwrap();

    // Increment the number of items migrated
    migration_state.items_migrated = migration_state
        .items_migrated
        .checked_add(1)
        .ok_or(MigrationError::Overflow)?;

    Ok(())
}
