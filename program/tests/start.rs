#![cfg(feature = "test-bpf")]
pub mod utils;

use mpl_migration_validator::error::MigrationError;
use mpl_migration_validator::instruction::UpdateArgs;
use mpl_migration_validator::{instruction::InitializeArgs, state::UnlockMethod};
use mpl_token_metadata::pda::find_collection_authority_account;
use mpl_token_metadata::state::{CollectionAuthorityRecord, TokenMetadataAccount};
use num_traits::FromPrimitive;
use solana_program::pubkey::Pubkey;
use solana_program_test::{tokio, BanksClientError};
use solana_sdk::{instruction::InstructionError, signer::Signer, transaction::TransactionError};
use utils::*;

#[tokio::test]
async fn start_migration() {
    let mut context = setup_context().await;

    // Create a default NFT to use as a collection.
    let nft = NfTest::new();
    nft.mint_default(&mut context, None).await.unwrap();

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
    println!("now: {}", now);
    let mut state = migratorr.state().clone();
    state.status.unlock_time = now as i64 - 2;

    // Set the state on the account.
    migratorr.inject_state(&mut context, state).await;

    // Warp ahead to ensure account is updated.
    context.warp_to_slot(100).unwrap();

    // Update the state account on-chain. This checks the current time
    // and updates the is_unlocked field if the wait time has passed.s
    let update_args = UpdateArgs { rule_set: None };

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
        find_collection_authority_account(&migratorr.mint(), &migratorr.delegate());

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
}
