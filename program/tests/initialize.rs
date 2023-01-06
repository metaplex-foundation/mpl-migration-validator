#![cfg(feature = "test-bpf")]
pub mod utils;

use borsh::BorshDeserialize;
use mpl_migration_validator::{
    instruction::{initialize, InitializeArgs},
    state::UnlockMethod,
};
use mpl_token_metadata::pda::find_metadata_account;
use num_traits::FromPrimitive;
use solana_program::pubkey::Pubkey;
use solana_program_test::{tokio, BanksClientError, ProgramTest};
use solana_sdk::signature::Keypair;
use solana_sdk::{
    instruction::InstructionError,
    signer::Signer,
    transaction::{Transaction, TransactionError},
};
use utils::find_migrate_state_pda;

use crate::utils::create_dummy_metadata_account;

const METADATA_RENT: u64 = 5616720;

#[tokio::test]
async fn initialize_successfully() {
    let mut test = ProgramTest::new("mpl_migration_validator", mpl_migration_validator::ID, None);

    let authority = Keypair::new();

    let dummy_rule_set = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();

    let (migrate_state_pubkey, _) = find_migrate_state_pda(mint_pubkey);
    let metadata_pubkey = find_metadata_account(&mint_pubkey).0;

    let metadata_account = create_dummy_metadata_account(mint_pubkey, authority.pubkey());

    test.add_account(metadata_pubkey, metadata_account);

    let mut context = test.start_with_context().await;

    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: Some(dummy_rule_set),
        unlock_method,
        collection_size: 0,
    };

    let instruction = initialize(
        context.payer.pubkey(),
        authority.pubkey(),
        mint_pubkey,
        metadata_pubkey,
        migrate_state_pubkey,
        args,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let migrate_state_account = context
        .banks_client
        .get_account(migrate_state_pubkey)
        .await
        .unwrap()
        .unwrap();

    let migrate_state = mpl_migration_validator::state::MigrationState::try_from_slice(
        &migrate_state_account.data[..],
    )
    .unwrap();

    assert_eq!(migrate_state.collection_info.mint, mint_pubkey);
    assert_eq!(migrate_state.collection_info.rule_set, dummy_rule_set);
}

#[tokio::test]
async fn cannot_initialize_twice() {
    let mut test = ProgramTest::new("mpl_migration_validator", mpl_migration_validator::ID, None);

    let authority = Keypair::new();

    let dummy_rule_set = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();

    let (migrate_state_pubkey, _) = find_migrate_state_pda(mint_pubkey);
    let metadata_pubkey = find_metadata_account(&mint_pubkey).0;

    let metadata_account = create_dummy_metadata_account(mint_pubkey, authority.pubkey());

    test.add_account(metadata_pubkey, metadata_account);

    let mut context = test.start_with_context().await;

    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: Some(dummy_rule_set),
        unlock_method,
        collection_size: 0,
    };

    let instruction = initialize(
        context.payer.pubkey(),
        authority.pubkey(),
        mint_pubkey,
        metadata_pubkey,
        migrate_state_pubkey,
        args,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let migrate_state_account = context
        .banks_client
        .get_account(migrate_state_pubkey)
        .await
        .unwrap()
        .unwrap();

    let migrate_state = mpl_migration_validator::state::MigrationState::try_from_slice(
        &migrate_state_account.data[..],
    )
    .unwrap();

    assert_eq!(migrate_state.collection_info.mint, mint_pubkey);
    assert_eq!(migrate_state.collection_info.rule_set, dummy_rule_set);

    // Try to initialize again with a different value

    let unlock_method = UnlockMethod::Vote;

    let args = InitializeArgs {
        rule_set: Some(dummy_rule_set),
        unlock_method,
        collection_size: 0,
    };

    let instruction = initialize(
        context.payer.pubkey(),
        authority.pubkey(),
        mint_pubkey,
        metadata_pubkey,
        migrate_state_pubkey,
        args,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err();

    assert_custom_error_ix!(0, err, 0x0);
}
