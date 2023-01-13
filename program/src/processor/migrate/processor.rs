#![allow(clippy::unreachable)]
#![allow(unreachable_code, unused_variables)]

use solana_program::program_pack::Pack;
use spl_token::state::Mint;

use crate::errors::GeneralError;

use super::*;

pub fn migrate_item<'a>(program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    return Err(GeneralError::FeatureDisabled.into());

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

    let account_context = AccountContext {
        program_id,
        payer_info,
        metadata_info,
        edition_info,
        mint_info,
        delegate_record_info,
        migration_state_info,
        program_signer_info,
        system_program_info,
        sysvar_instructions_info,
        spl_token_program_info,
        token_metadata_program_info,
    };

    // Validate Accounts
    validate_accounts(&account_context)?;

    // Deserialize accounts
    let metadata = Metadata::from_account_info(metadata_info)
        .map_err(|_| DeserializationError::InvalidMetadata)?;

    let collection_metadata = Metadata::from_account_info(collection_metadata_info)
        .map_err(|_| DeserializationError::InvalidMetadata)?;

    let mut migration_state = MigrationState::from_account_info(migration_state_info)?;

    let mint = Mint::unpack(&mint_info.data.borrow())?;
    let token = Account::unpack(&token_info.data.borrow())?;

    let program_signer = ProgramSigner::from_account_info(program_signer_info)?;
    let signers_seeds = &[b"signer", crate::ID.as_ref(), &[program_signer.bump]];

    let data_context = DataContext {
        metadata: &metadata,
        collection_metadata: &collection_metadata,
        migration_state: &migration_state,
        mint: &mint,
        token: &token,
    };

    // Validate relatonships between accounts
    validate_relationships(&account_context, &data_context)?;

    // Validate the delegate record is correct.
    validate_delegate(&account_context, &data_context)?;

    // Validate this item passes all eligibility rules.
    validate_eligibility(&account_context, &data_context)?;

    // Migrate the item by CPI'ing into Token Metadata.
    let args = MigrateArgs::V1 {
        migration_type: MigrationType::ProgrammableV1,
        rule_set: Some(migration_state.collection_info.rule_set),
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
        .map_err(|_| GeneralError::InvalidInstruction)?
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
    migration_state.status.items_migrated = migration_state
        .status
        .items_migrated
        .checked_add(1)
        .ok_or(GeneralError::Overflow)?;

    Ok(())
}
