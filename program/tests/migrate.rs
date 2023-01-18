#![cfg(feature = "test-bpf")]
pub mod utils;

use mpl_migration_validator::instruction::UpdateArgs;
use mpl_migration_validator::state::ProgramSigner;
use mpl_migration_validator::{instruction::InitializeArgs, state::UnlockMethod};
use mpl_token_metadata::pda::find_collection_authority_account;
use mpl_token_metadata::state::{CollectionAuthorityRecord, TokenMetadataAccount, TokenStandard};
use solana_program_test::tokio;
use solana_sdk::signer::Signer;
use utils::*;

#[tokio::test]
async fn migrate_collection_success() {
    let mut context = setup_context().await;

    let collection_authority = context.payer.dirty_clone();

    // We create a collection with five NFTs in it.
    let mut collection_nft = NfTest::new();
    collection_nft
        .mint_default(&mut context, None)
        .await
        .unwrap();

    let mut nft1 = NfTest::new();
    nft1.mint_default(&mut context, None).await.unwrap();
    nft1.set_and_verify_collection(
        &mut context,
        collection_nft.metadata_pubkey(),
        &collection_authority,
        collection_authority.pubkey(),
        collection_nft.mint_pubkey(),
        collection_nft.edition_pubkey().unwrap(),
        None,
    )
    .await
    .unwrap();

    let mut nft2 = NfTest::new();
    nft2.mint_default(&mut context, None).await.unwrap();
    nft2.set_and_verify_collection(
        &mut context,
        collection_nft.metadata_pubkey(),
        &collection_authority,
        collection_authority.pubkey(),
        collection_nft.mint_pubkey(),
        collection_nft.edition_pubkey().unwrap(),
        None,
    )
    .await
    .unwrap();

    let mut nft3 = NfTest::new();
    nft3.mint_default(&mut context, None).await.unwrap();
    nft3.set_and_verify_collection(
        &mut context,
        collection_nft.metadata_pubkey(),
        &collection_authority,
        collection_authority.pubkey(),
        collection_nft.mint_pubkey(),
        collection_nft.edition_pubkey().unwrap(),
        None,
    )
    .await
    .unwrap();

    let payer = context.payer.dirty_clone();

    // Create our migration state manager.
    let mut migratorr = Migratorr::new(collection_nft.mint_pubkey());

    // Initialize the program signer
    migratorr.init_signer(&mut context, &payer).await.unwrap();

    let args = InitializeArgs {
        rule_set: None, // this defaults to the default public key
        unlock_method: UnlockMethod::Timed,
        collection_size: 3,
    };

    // Initialize the migration state account on-chain
    migratorr
        .initialize(&mut context, &payer, &payer, &collection_nft, args)
        .await
        .unwrap();

    // Refresh the migratorr's state from the on-chain account.
    migratorr.refresh_state(&mut context).await.unwrap();

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
        .update(&mut context, &payer, update_args)
        .await
        .unwrap();

    // Now we try to start the migration expecting it to succeed.
    migratorr
        .start(&mut context, &payer, &payer, &collection_nft)
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

    // We are ready to migrate.

    let token_owner = context.payer.pubkey();

    // NFT 1
    migratorr
        .migrate_item(
            &mut context,
            &payer,
            collection_nft.mint_pubkey(),
            token_owner,
            &nft1,
        )
        .await
        .unwrap();

    migratorr
        .migrate_item(
            &mut context,
            &payer,
            collection_nft.mint_pubkey(),
            token_owner,
            &nft2,
        )
        .await
        .unwrap();

    migratorr
        .migrate_item(
            &mut context,
            &payer,
            collection_nft.mint_pubkey(),
            token_owner,
            &nft3,
        )
        .await
        .unwrap();

    let nft1_md = nft1.get_data(&mut context).await;
    let nft2_md = nft1.get_data(&mut context).await;
    let nft3_md = nft1.get_data(&mut context).await;

    // Correct token standard
    assert_eq!(
        nft1_md.token_standard,
        Some(TokenStandard::ProgrammableNonFungible)
    );
    assert_eq!(
        nft2_md.token_standard,
        Some(TokenStandard::ProgrammableNonFungible)
    );
    assert_eq!(
        nft3_md.token_standard,
        Some(TokenStandard::ProgrammableNonFungible)
    );
}
