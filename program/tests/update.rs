#![cfg(feature = "test-bpf")]
pub mod utils;

use mpl_migration_validator::instruction::UpdateArgs;
use mpl_migration_validator::state::UnlockMethod;
use mpl_migration_validator::{self, instruction::InitializeArgs};
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::utils::*;

#[tokio::test]
async fn update_rule_set() {
    let mut context = setup_context().await;

    // Create an authority that is separate from the payer.
    let authority = Keypair::new();
    authority
        .airdrop(&mut context, 1_000_000_000)
        .await
        .unwrap();

    // Create a default NFT to use as a collection.
    let mut nft = NfTest::new();
    nft.mint_default(&mut context, Some(&authority))
        .await
        .unwrap();

    // Create our migration state manager.
    let mut migratorr = Migratorr::new(nft.mint_pubkey());

    // Set up our initialize args
    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: None, // this defaults to the default public key
        unlock_method,
        collection_size: 0,
    };

    let payer = context.payer.dirty_clone();

    // Initialize the migration state account on-chain
    migratorr
        .initialize(&mut context, &payer, &authority, &nft, args)
        .await
        .unwrap();

    // Refresh the migratorr's state from the on-chain account.
    migratorr.refresh_state(&mut context).await.unwrap();

    assert_eq!(migratorr.mint(), nft.mint_pubkey());
    assert_eq!(migratorr.authority(), authority.pubkey());

    let dummy_rule_set = Pubkey::new_unique();
    let update_args = UpdateArgs {
        rule_set: Some(dummy_rule_set),
        collection_size: None,
    };

    migratorr
        .update(&mut context, &authority, None, update_args)
        .await
        .unwrap();

    migratorr.refresh_state(&mut context).await.unwrap();

    assert_eq!(migratorr.rule_set(), dummy_rule_set);
}

#[tokio::test]
async fn update_collection_size() {
    let mut context = setup_context().await;

    // Create an authority that is separate from the payer.
    let authority = Keypair::new();
    authority
        .airdrop(&mut context, 1_000_000_000)
        .await
        .unwrap();

    // Create a default NFT to use as a collection.
    let mut nft = NfTest::new();
    nft.mint_default(&mut context, Some(&authority))
        .await
        .unwrap();

    // Create our migration state manager.
    let mut migratorr = Migratorr::new(nft.mint_pubkey());

    // Set up our initialize args
    let unlock_method = UnlockMethod::Timed;

    let args = InitializeArgs {
        rule_set: None, // this defaults to the default public key
        unlock_method,
        collection_size: 0,
    };

    let payer = context.payer.dirty_clone();

    // Initialize the migration state account on-chain
    migratorr
        .initialize(&mut context, &payer, &authority, &nft, args)
        .await
        .unwrap();

    // Refresh the migratorr's state from the on-chain account.
    migratorr.refresh_state(&mut context).await.unwrap();

    assert_eq!(migratorr.mint(), nft.mint_pubkey());
    assert_eq!(migratorr.authority(), authority.pubkey());

    let new_collection_size = 888;
    let update_args = UpdateArgs {
        rule_set: None,
        collection_size: Some(new_collection_size),
    };

    migratorr
        .update(&mut context, &authority, None, update_args)
        .await
        .unwrap();

    migratorr.refresh_state(&mut context).await.unwrap();

    assert_eq!(migratorr.collection_size(), new_collection_size);
}
