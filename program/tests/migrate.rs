#![cfg(feature = "test-bpf")]
pub mod utils;

use mpl_migration_validator::state::ProgramSigner;
use mpl_migration_validator::{instruction::InitializeArgs, state::UnlockMethod};
use mpl_token_metadata::pda::find_collection_authority_account;
use mpl_token_metadata::state::{
    CollectionAuthorityRecord, TokenDelegateRole, TokenMetadataAccount, TokenState,
};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program_test::tokio;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use utils::*;

mod eligible_scenarios {
    use super::*;

    #[tokio::test]
    async fn migrate_collection_success() {
        // The happy path for migrating a collection.
        // No SPL token delegates and tokens start unfrozen.
        //
        // Success States:
        // Token Standard:       ProgrammableNonFungible
        // Token Freeze State:   Frozen
        // TokenRecord Delegate: None
        // TokenRecord Role:     None
        // TokenRecord State:    Unlocked
        let mut context = setup_pnft_context().await;

        let collection_authority = context.payer.dirty_clone();

        // We create a collection with three NFTs in it.
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

        // Create default rule set to apply to migrated NFTs.
        let (rule_set, _auth_rules) = create_default_metaplex_rule_set(&mut context, payer).await;

        // Create our migration state manager.
        let mut migratorr = Migratorr::new(collection_nft.mint_pubkey());

        let payer = context.payer.dirty_clone();

        // Initialize the program signer
        migratorr.init_signer(&mut context, &payer).await.unwrap();

        let args = InitializeArgs {
            rule_set: Some(rule_set),
            unlock_method: UnlockMethod::Timed,
            collection_size: 3,
        };

        // Initialize the migration state account on-chain
        migratorr
            .initialize(&mut context, &payer, &payer, &collection_nft, args)
            .await
            .unwrap();

        // Artificially update the timestamp to allow the migration to start
        // and call update to unlock the collection.
        migratorr
            .unlock_collection(&mut context, &collection_authority)
            .await;

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
        assert_eq!(migratorr.rule_set(), rule_set);

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

        // The rule set should match.
        // NFTs had no SPL token delegate so their delegate and role should be None.
        // The NFTs should be unlocked because they weren't frozen initially.
        nft1.assert_pnft_migration(
            &mut context,
            Some(rule_set),
            None,
            None,
            TokenState::Unlocked,
        )
        .await
        .unwrap();

        nft2.assert_pnft_migration(
            &mut context,
            Some(rule_set),
            None,
            None,
            TokenState::Unlocked,
        )
        .await
        .unwrap();

        nft3.assert_pnft_migration(
            &mut context,
            Some(rule_set),
            None,
            None,
            TokenState::Unlocked,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn unfrozen_with_spl_delegate() {
        // Migrate an unfrozen NFT with an SPL token delegate.
        // This should successfully migrate the NFT to a pNFT and create
        // a TokenRecord with a MigrationDelegate.
        // Its state should be Unlocked since it started unfrozen.
        // The NFT token account should be frozen after migration.
        //
        // Success States:
        // Token Standard:       ProgrammableNonFungible
        // Token Freeze State:   Frozen
        // TokenRecord Delegate: SPL Token Delegate
        // TokenRecord Role:     MigrationDelegate
        // TokenRecord State:    Unlocked
        let mut context = setup_pnft_context().await;

        let collection_authority = context.payer.dirty_clone();

        // We create a collection to contain the NFT.
        let mut collection_nft = NfTest::new();
        collection_nft
            .mint_default(&mut context, None)
            .await
            .unwrap();

        let mut nft = NfTest::new();
        nft.mint_default(&mut context, None).await.unwrap();
        nft.set_and_verify_collection(
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

        // Create default rule set to apply to migrated NFTs.
        let (rule_set, _auth_rules) = create_default_metaplex_rule_set(&mut context, payer).await;

        // Create our migration state manager.
        let mut migratorr = Migratorr::new(collection_nft.mint_pubkey());

        let payer = context.payer.dirty_clone();

        // Initialize the program signer
        migratorr.init_signer(&mut context, &payer).await.unwrap();

        let args = InitializeArgs {
            rule_set: Some(rule_set), // this defaults to the default public key
            unlock_method: UnlockMethod::Timed,
            collection_size: 1,
        };

        // Initialize the migration state account on-chain
        migratorr
            .initialize(&mut context, &payer, &payer, &collection_nft, args)
            .await
            .unwrap();

        // Artificially update the timestamp to allow the migration to start
        // and call update to unlock the collection.
        migratorr
            .unlock_collection(&mut context, &collection_authority)
            .await;

        // Enable migration
        migratorr
            .start(&mut context, &payer, &payer, &collection_nft)
            .await
            .unwrap();

        // Assign a spl token delegate to the NFT
        let delegate = Keypair::new();

        nft.spl_delegate(&mut context, &payer, &delegate.pubkey())
            .await
            .unwrap();

        let token_owner = context.payer.pubkey();

        // Migrate the NFT
        migratorr
            .migrate_item(
                &mut context,
                &payer,
                collection_nft.mint_pubkey(),
                token_owner,
                &nft,
            )
            .await
            .unwrap();

        nft.assert_pnft_migration(
            &mut context,
            Some(rule_set),
            Some(delegate.pubkey()),
            // TODO: refactor to MigrationDelegate once that's enabled in TokenMetadata
            Some(TokenDelegateRole::Utility),
            TokenState::Unlocked,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn frozen_with_spl_delegate() {
        // Migrate an already frozen NFT with an SPL token delegate.
        // This should successfully migrate the NFT to a pNFT and create
        // a TokenRecord with a MigrationDelegate.
        // Its state should be Locked since it started frozen.
        // The NFT token account should be frozen after migration.
        //
        // Success States:
        // Token Standard:       ProgrammableNonFungible
        // Token Freeze State:   Frozen
        // TokenRecord Delegate: SPL Token Delegate
        // TokenRecord Role:     MigrationDelegate
        // TokenRecord State:    Locked
        let mut context = setup_pnft_context().await;

        let collection_authority = context.payer.dirty_clone();

        // We create a collection to contain the NFT.
        let mut collection_nft = NfTest::new();
        collection_nft
            .mint_default(&mut context, None)
            .await
            .unwrap();

        let mut nft = NfTest::new();
        nft.mint_default(&mut context, None).await.unwrap();
        nft.set_and_verify_collection(
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

        // Create default rule set to apply to migrated NFTs.
        let (rule_set, _auth_rules) = create_default_metaplex_rule_set(&mut context, payer).await;

        // Create our migration state manager.
        let mut migratorr = Migratorr::new(collection_nft.mint_pubkey());

        let payer = context.payer.dirty_clone();

        // Initialize the program signer
        migratorr.init_signer(&mut context, &payer).await.unwrap();

        let args = InitializeArgs {
            rule_set: Some(rule_set), // this defaults to the default public key
            unlock_method: UnlockMethod::Timed,
            collection_size: 1,
        };

        // Initialize the migration state account on-chain
        migratorr
            .initialize(&mut context, &payer, &payer, &collection_nft, args)
            .await
            .unwrap();

        // Artificially update the timestamp to allow the migration to start
        // and call update to unlock the collection.
        migratorr
            .unlock_collection(&mut context, &collection_authority)
            .await;

        // Enable migration
        migratorr
            .start(&mut context, &payer, &payer, &collection_nft)
            .await
            .unwrap();

        // Assign a spl token delegate to the NFT
        let owner = context.payer.dirty_clone();
        let delegate = Keypair::new();
        delegate
            .airdrop(&mut context, LAMPORTS_PER_SOL)
            .await
            .unwrap();

        nft.spl_delegate(&mut context, &payer, &delegate.pubkey())
            .await
            .unwrap();

        nft.refresh_accounts(&mut context).await.unwrap();

        nft.freeze_token(&mut context, &delegate).await.unwrap();

        // Migrate the NFT
        migratorr
            .migrate_item(
                &mut context,
                &payer,
                collection_nft.mint_pubkey(),
                owner.pubkey(),
                &nft,
            )
            .await
            .unwrap();

        nft.assert_pnft_migration(
            &mut context,
            Some(rule_set),
            Some(delegate.pubkey()),
            Some(TokenDelegateRole::Utility),
            TokenState::Locked,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn frozen_with_no_spl_delegate() {
        // Migrate an already frozen NFT without an SPL token delegate.
        // This should not be a possible scenario under current delegate and freeze rules
        // but if encountered it can simply be migrated without a delegate and kept frozen.
        // Even though it started frozen, its state should be Unlocked since it has
        // no delegate to unlock it.
        //
        // Success States:
        // Token Standard:       ProgrammableNonFungible
        // Token Freeze State:   Frozen
        // TokenRecord Delegate: None
        // TokenRecord Role:     None
        // TokenRecord State:    Unlocked
        let mut context = setup_pnft_context().await;

        let collection_authority = context.payer.dirty_clone();

        // We create a collection to contain the NFT.
        let mut collection_nft = NfTest::new();
        collection_nft
            .mint_default(&mut context, None)
            .await
            .unwrap();

        let mut nft = NfTest::new();
        nft.mint_default(&mut context, None).await.unwrap();
        nft.set_and_verify_collection(
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

        // Create default rule set to apply to migrated NFTs.
        let (rule_set, _auth_rules) = create_default_metaplex_rule_set(&mut context, payer).await;

        // Create our migration state manager.
        let mut migratorr = Migratorr::new(collection_nft.mint_pubkey());

        let payer = context.payer.dirty_clone();

        // Initialize the program signer
        migratorr.init_signer(&mut context, &payer).await.unwrap();

        let args = InitializeArgs {
            rule_set: Some(rule_set), // this defaults to the default public key
            unlock_method: UnlockMethod::Timed,
            collection_size: 1,
        };

        // Initialize the migration state account on-chain
        migratorr
            .initialize(&mut context, &payer, &payer, &collection_nft, args)
            .await
            .unwrap();

        // Artificially update the timestamp to allow the migration to start
        // and call update to unlock the collection.
        migratorr
            .unlock_collection(&mut context, &collection_authority)
            .await;

        // Enable migration
        migratorr
            .start(&mut context, &payer, &payer, &collection_nft)
            .await
            .unwrap();

        // Assign a spl token delegate to the NFT
        let owner = context.payer.dirty_clone();

        // Simulate a frozen NFT with no delegate by directly injecting the frozen state.
        nft.inject_frozen_state(&mut context).await;

        // Migrate the NFT
        migratorr
            .migrate_item(
                &mut context,
                &payer,
                collection_nft.mint_pubkey(),
                owner.pubkey(),
                &nft,
            )
            .await
            .unwrap();

        nft.assert_pnft_migration(
            &mut context,
            Some(rule_set),
            None,
            None,
            TokenState::Unlocked,
        )
        .await
        .unwrap();
    }
}
