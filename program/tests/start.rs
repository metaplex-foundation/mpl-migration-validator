#![cfg(feature = "test-bpf")]
pub mod utils;

use mpl_migration_validator::errors::{DeserializationError, MigrationError, ValidationError};
use mpl_migration_validator::instruction::{start, UpdateArgs};
use mpl_migration_validator::state::ProgramSigner;
use mpl_migration_validator::{instruction::InitializeArgs, state::UnlockMethod};
use mpl_token_metadata::pda::{find_collection_authority_account, find_metadata_account};
use mpl_token_metadata::state::{CollectionAuthorityRecord, TokenMetadataAccount};
use num_traits::FromPrimitive;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program_test::{tokio, BanksClientError, ProgramTest};
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    account::Account, instruction::InstructionError, signer::Signer, transaction::TransactionError,
};
use utils::*;

#[tokio::test]
async fn start_migration() {
    let mut context = setup_context().await;

    // Create a default NFT to use as a collection.
    let mut nft = NfTest::new();
    nft.mint_default(&mut context, None).await.unwrap();

    let mut nft2 = NfTest::new();
    nft2.mint_default(&mut context, None).await.unwrap();

    // Create our migration state manager.
    let mut migratorr = Migratorr::new(nft.mint_pubkey());

    // Set up our initialize args
    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: None, // this defaults to the default public key
        unlock_method: UnlockMethod::Timed,
        collection_size: 0,
    };

    let payer = context.payer.dirty_clone();

    // Initialize the migration state account on-chain
    migratorr
        .initialize(&mut context, &payer, &payer, &nft, args)
        .await
        .unwrap();

    // Refresh the migratorr's state from the on-chain account.
    migratorr.refresh_state(&mut context).await.unwrap();

    // Check values are as expected.
    assert_eq!(migratorr.authority(), payer.pubkey());
    assert_eq!(migratorr.rule_set(), Pubkey::default());
    assert_eq!(migratorr.unlock_method(), unlock_method);
    assert_eq!(migratorr.collection_size(), 0);
    assert_eq!(migratorr.mint(), nft.mint_pubkey());

    // First we try to start the migration expecting it to fail because
    // the current time will not be greater than or equal to the unlock
    // time.
    let err = migratorr
        .start(&mut context, &payer, &payer, &nft)
        .await
        .unwrap_err();

    assert_custom_error_ix!(0, err, MigrationError::MigrationLocked);

    // We need to inject the account with the state set to a timestamp
    // that allows our migration to start.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;

    let mut state = migratorr.state().clone();
    state.status.unlock_time = now as i64 - 2;

    // Set the state on the account.
    migratorr.inject_state(&mut context, state).await;

    // Warp ahead to ensure account is updated.
    context.warp_to_slot(100).unwrap();

    // Update the state account on-chain. This checks the current time
    // and updates the is_unlocked field if the wait time has passed.s
    let update_args = UpdateArgs {
        rule_set: None,
        collection_size: None,
    };

    migratorr
        .update(&mut context, &payer, None, update_args)
        .await
        .unwrap();

    // Now we try to start the migration expecting it to succeed.
    migratorr
        .start(&mut context, &payer, &payer, &nft)
        .await
        .unwrap();

    // Refresh the migratorr's state from the on-chain account.
    migratorr.refresh_state(&mut context).await.unwrap();

    // Check values are as expected.
    assert!(migratorr.state().status.in_progress);
    assert!(!migratorr.state().status.is_locked);

    // Ensure the collection delegate was created.
    let (delegate_record_pda, bump) =
        find_collection_authority_account(&migratorr.mint(), &ProgramSigner::pubkey());

    // This function call panics if the account doesn't exist.
    let delegate_record_account = get_account(&mut context, &delegate_record_pda).await;

    let delegate_record =
        CollectionAuthorityRecord::safe_deserialize(delegate_record_account.data.as_slice())
            .expect("Failed to deserialize delegate record account");

    // Check authority and bump values are as expected.
    assert_eq!(
        delegate_record.update_authority.unwrap(),
        migratorr.authority()
    );
    assert_eq!(delegate_record.bump, bump);
    // Record matches what was stored in the migration state.
    assert_eq!(migratorr.delegate_record(), delegate_record_pda);

    context.warp_to_slot(200).unwrap();

    // Running start again should fail because the migration is already in progress.
    let err = migratorr
        .start(&mut context, &payer, &payer, &nft)
        .await
        .unwrap_err();

    assert_custom_error_ix!(0, err, MigrationError::MigrationInProgress);
}

#[tokio::test]
async fn wrong_authority_fails() {
    let mut context = setup_context().await;

    let fake_authority = Keypair::new();

    // Create a default NFT to use as a collection.
    let mut nft = NfTest::new();
    nft.mint_default(&mut context, None).await.unwrap();

    // Create our migration state manager.
    let migratorr = Migratorr::new(nft.mint_pubkey());

    // Set up our initialize args
    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: None, // this defaults to the default public key
        unlock_method,
        collection_size: 0,
    };

    let payer = context.payer.dirty_clone();

    // Initialize the migration state account on-chain
    let err = migratorr
        .initialize(&mut context, &payer, &fake_authority, &nft, args)
        .await
        .unwrap_err();

    assert_custom_error_ix!(0, err, ValidationError::InvalidAuthority);
}

#[tokio::test]
async fn incorrect_migration_state_fails() {
    let mut context = setup_context().await;

    let other_authority = Keypair::new();
    other_authority
        .airdrop(&mut context, 1_000_000_000)
        .await
        .unwrap();

    // Create a default NFT to use as a collection.
    let mut nft = NfTest::new();
    nft.mint_default(&mut context, None).await.unwrap();

    // NFT for someone else's migration state account.
    let mut other_nft = NfTest::new();
    other_nft
        .mint_default(&mut context, Some(&other_authority))
        .await
        .unwrap();

    // Create our migration state managers.
    let migratorr = Migratorr::new(nft.mint_pubkey());
    let other_migratorr = Migratorr::new(other_nft.mint_pubkey());

    // Set up our initialize args
    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: None, // this defaults to the default public key
        unlock_method,
        collection_size: 0,
    };

    let payer = context.payer.dirty_clone();

    // Initialize both states.
    migratorr
        .initialize(&mut context, &payer, &payer, &nft, args.clone())
        .await
        .unwrap();

    other_migratorr
        .initialize(&mut context, &payer, &other_authority, &other_nft, args)
        .await
        .unwrap();

    let delegate = ProgramSigner::pubkey();
    let (delegate_record, _) = find_collection_authority_account(&nft.mint_pubkey(), &delegate);

    let instruction = start(
        payer.pubkey(),
        payer.pubkey(),
        nft.mint_pubkey(),
        nft.metadata_pubkey(),
        delegate,
        delegate_record,
        other_migratorr.pubkey(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err();

    assert_custom_error_ix!(0, err, ValidationError::InvalidMigrationStateDerivation);
}

#[tokio::test]
async fn zeroed_state_account() {
    let mut test = ProgramTest::new("mpl_migration_validator", mpl_migration_validator::ID, None);
    test.add_program("mpl_token_metadata", mpl_token_metadata::ID, None);

    let mut context = test.start_with_context().await;

    // This attack relies on creating accounts full of zeros and reassigning
    // them to the migratorr program.
    //
    // We simulate a malicious program which creates an empty account the same
    // length as a metadata account and assigns it be owned by token-metadata
    // by injecting an account into our context.
    let mint = Keypair::new();

    let mint_lamports = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(spl_token::state::Mint::LEN);

    let ix_create_account = system_instruction::create_account(
        &context.payer.pubkey(),
        &mint.pubkey(),
        mint_lamports,
        spl_token::state::Mint::LEN as u64,
        &spl_token::ID,
    );

    let ix_create_mint = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &mint.pubkey(),
        None,
        0,
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[ix_create_account, ix_create_mint],
        Some(&context.payer.pubkey()),
        &[&context.payer, &mint],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let (empty_metadata_pubkey, _bump) = find_metadata_account(&mint.pubkey());

    warp100(&mut context).await;

    let lamports = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(679);

    let empty_metadata_account = Account {
        lamports,
        data: vec![0; 679],
        owner: mpl_token_metadata::ID,
        executable: false,
        rent_epoch: 0,
    };

    context.set_account(&empty_metadata_pubkey, &empty_metadata_account.into());
    warp100(&mut context).await;

    // Now our malicious program constructs an empty migration state account
    // and assigns it to the mpl-migration-validator program.
    let (empty_migrate_state_pubkey, _bump) = find_migrate_state_pda(&mint.pubkey());

    warp100(&mut context).await;

    let lamports = context
        .banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(679);

    let empty_migrate_state_account = Account {
        lamports,
        data: vec![0; 679],
        owner: mpl_migration_validator::ID,
        executable: false,
        rent_epoch: 0,
    };

    context.set_account(
        &empty_migrate_state_pubkey,
        &empty_migrate_state_account.into(),
    );

    warp100(&mut context).await;

    let mut nft = NfTest::new();
    nft.set_mint(mint.dirty_clone());
    nft.set_metadata(empty_metadata_pubkey);

    let authority = context.payer.dirty_clone();

    let migratorr = Migratorr::new(mint.pubkey());
    let err = migratorr
        .start(&mut context, &authority, &authority, &nft)
        .await
        .unwrap_err();

    // This will be the first error it encounters.
    assert_custom_error_ix!(0, err, DeserializationError::ZeroedMigrationState);
}
