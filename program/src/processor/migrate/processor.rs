use mpl_token_metadata::instruction::MetadataInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
};
use spl_token::state::Mint;

use crate::errors::GeneralError;

use super::*;

pub fn migrate_item<'a>(program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    msg!("Migrate Item");
    // Fetch accounts
    let account_info_iter = &mut accounts.iter();
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let token_info = next_account_info(account_info_iter)?;
    let token_owner_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let program_signer_info = next_account_info(account_info_iter)?;
    let collection_metadata_info = next_account_info(account_info_iter)?;
    let delegate_record_info = next_account_info(account_info_iter)?;
    let token_record_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let sysvar_instructions_info = next_account_info(account_info_iter)?;
    let spl_token_program_info = next_account_info(account_info_iter)?;
    let mpl_token_auth_rules_program_info = next_account_info(account_info_iter)?;
    let auth_rule_set_info = next_account_info(account_info_iter)?;
    let migration_state_info = next_account_info(account_info_iter)?;
    let token_metadata_program_info = next_account_info(account_info_iter)?;

    let ctx = AccountContext {
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
    validate_accounts(&ctx)?;

    // Deserialize accounts
    let metadata = Metadata::from_account_info(ctx.metadata_info)
        .map_err(|_| DeserializationError::InvalidMetadata)?;

    let collection_metadata = Metadata::from_account_info(collection_metadata_info)
        .map_err(|_| DeserializationError::InvalidMetadata)?;

    let mut migration_state = MigrationState::from_account_info(ctx.migration_state_info)?;

    let mint = Mint::unpack(&ctx.mint_info.data.borrow())?;
    let token = Account::unpack(&token_info.data.borrow())?;

    let program_signer = ProgramSigner::from_account_info(ctx.program_signer_info)?;
    let signers_seeds: &[&[u8]] = &[b"signer", &[program_signer.bump]];

    let data_context = DataContext {
        metadata: &metadata,
        collection_metadata: &collection_metadata,
        migration_state: &migration_state,
        mint: &mint,
        token: &token,
    };

    // Validate relatonships between accounts
    validate_relationships(&ctx, &data_context)?;

    // Validate the delegate record is correct.
    validate_delegate(&ctx, &data_context)?;

    // Validate this item passes all eligibility rules.
    validate_eligibility(&ctx, &data_context)?;

    // Migrate the item by CPI'ing into Token Metadata.
    let args = MigrateArgs::V1 {
        migration_type: MigrationType::ProgrammableV1,
        rule_set: Some(migration_state.collection_info.rule_set),
    };

    let account_infos = vec![
        metadata_info.clone(),
        edition_info.clone(),
        token_info.clone(),
        token_owner_info.clone(),
        mint_info.clone(),
        payer_info.clone(),
        program_signer_info.clone(),
        collection_metadata_info.clone(),
        delegate_record_info.clone(),
        token_record_info.clone(),
        system_program_info.clone(),
        sysvar_instructions_info.clone(),
        spl_token_program_info.clone(),
        mpl_token_auth_rules_program_info.clone(),
        auth_rule_set_info.clone(),
    ];

    let accounts = vec![
        AccountMeta::new(*metadata_info.key, false),
        AccountMeta::new(*edition_info.key, false),
        AccountMeta::new(*token_info.key, false),
        AccountMeta::new_readonly(*token_owner_info.key, false),
        AccountMeta::new_readonly(*mint_info.key, false),
        AccountMeta::new(*payer_info.key, true),
        AccountMeta::new_readonly(*program_signer_info.key, true),
        AccountMeta::new_readonly(*collection_metadata_info.key, false),
        AccountMeta::new_readonly(*delegate_record_info.key, false),
        AccountMeta::new(*token_record_info.key, false),
        AccountMeta::new_readonly(*system_program_info.key, false),
        AccountMeta::new_readonly(*sysvar_instructions_info.key, false),
        AccountMeta::new_readonly(*spl_token_program_info.key, false),
        AccountMeta::new_readonly(*mpl_token_auth_rules_program_info.key, false),
        AccountMeta::new_readonly(*auth_rule_set_info.key, false),
    ];

    let instruction = Instruction {
        program_id: mpl_token_metadata::ID,
        accounts,
        data: MetadataInstruction::Migrate(args).try_to_vec().unwrap(),
    };

    invoke_signed(&instruction, &account_infos, &[signers_seeds]).unwrap();

    // Increment the number of items migrated
    migration_state.status.items_migrated = migration_state
        .status
        .items_migrated
        .checked_add(1)
        .ok_or(GeneralError::Overflow)?;

    Ok(())
}
