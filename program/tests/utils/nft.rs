use mpl_token_metadata::{
    id, instruction,
    state::{Collection, CollectionDetails, Creator, Uses, EDITION, PREFIX},
};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::{
    pubkey::Pubkey, signature::Signer, signer::keypair::Keypair, transaction::Transaction,
};

use super::*;

#[derive(Debug)]
pub struct NfTest {
    mint: Keypair,
    metadata: Pubkey,
    token: Keypair,
    edition: Option<Pubkey>,
}

impl NfTest {
    pub fn new() -> Self {
        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();
        let program_id = id();

        let metadata_seeds = &[PREFIX.as_bytes(), program_id.as_ref(), mint_pubkey.as_ref()];
        let (metadata, _) = Pubkey::find_program_address(metadata_seeds, &id());

        NfTest {
            mint,
            metadata,
            token: Keypair::new(),
            edition: None,
        }
    }

    pub fn mint_pubkey(&self) -> Pubkey {
        self.mint.pubkey()
    }

    pub fn mint_keypair(&self) -> &Keypair {
        &self.mint
    }

    pub fn metadata_pubkey(&self) -> Pubkey {
        self.metadata
    }

    pub fn token_pubkey(&self) -> Pubkey {
        self.token.pubkey()
    }

    pub fn token_keypair(&self) -> &Keypair {
        &self.token
    }

    pub async fn get_data(
        &self,
        context: &mut ProgramTestContext,
    ) -> mpl_token_metadata::state::Metadata {
        let account = get_account(context, &self.metadata).await;
        try_from_slice_unchecked(&account.data).unwrap()
    }

    pub async fn mint(
        &self,
        context: &mut ProgramTestContext,
        authority: &Keypair,
        name: String,
        symbol: String,
        uri: String,
        creators: Option<Vec<Creator>>,
        seller_fee_basis_points: u16,
        is_mutable: bool,
        collection: Option<Collection>,
        uses: Option<Uses>,
        collection_details: Option<CollectionDetails>,
    ) -> Result<(), BanksClientError> {
        create_mint(
            context,
            &self.mint,
            &authority.pubkey(),
            Some(&authority.pubkey()),
            0,
        )
        .await?;
        create_token_account(
            context,
            &self.token,
            &self.mint.pubkey(),
            &authority.pubkey(),
        )
        .await?;
        mint_tokens(
            context,
            &self.mint.pubkey(),
            &self.token.pubkey(),
            1,
            &authority,
            None,
        )
        .await?;

        let tx = Transaction::new_signed_with_payer(
            &[instruction::create_metadata_accounts_v3(
                id(),
                self.metadata,
                self.mint.pubkey(),
                authority.pubkey(),
                context.payer.pubkey(),
                authority.pubkey(),
                name,
                symbol,
                uri,
                creators,
                seller_fee_basis_points,
                false,
                is_mutable,
                collection,
                uses,
                collection_details,
            )],
            Some(&authority.pubkey()),
            &[&context.payer, authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn mint_default(
        &mut self,
        context: &mut ProgramTestContext,
        authority: Option<&Keypair>,
    ) -> Result<(), BanksClientError> {
        self.mint(
            context,
            authority.unwrap_or(&context.payer.dirty_clone()),
            "name".to_string(),
            "symbol".to_string(),
            "uri".to_string(),
            None,
            0,
            false,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        let master_edition = MasterEditionV2::new(&self);
        master_edition
            .create_v3(context, authority, Some(0))
            .await
            .unwrap();

        self.edition = Some(master_edition.pubkey);

        Ok(())
    }
}

impl Default for NfTest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct MasterEditionV2 {
    pub pubkey: Pubkey,
    pub metadata_pubkey: Pubkey,
    pub mint_pubkey: Pubkey,
}

impl MasterEditionV2 {
    pub fn new(nft: &NfTest) -> Self {
        let program_id = id();
        let mint_pubkey = nft.mint.pubkey();

        let master_edition_seeds = &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            mint_pubkey.as_ref(),
            EDITION.as_bytes(),
        ];
        let (pubkey, _) = Pubkey::find_program_address(master_edition_seeds, &id());

        MasterEditionV2 {
            pubkey,
            metadata_pubkey: nft.metadata,
            mint_pubkey,
        }
    }

    pub async fn get_data_from_account(
        context: &mut ProgramTestContext,
        pubkey: &Pubkey,
    ) -> mpl_token_metadata::state::MasterEditionV2 {
        let account = get_account(context, pubkey).await;
        try_from_slice_unchecked(&account.data).unwrap()
    }

    pub async fn create_v3(
        &self,
        context: &mut ProgramTestContext,
        authority: Option<&Keypair>,
        max_supply: Option<u64>,
    ) -> Result<(), BanksClientError> {
        let authority = if let Some(auth) = authority {
            auth
        } else {
            &context.payer
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction::create_master_edition_v3(
                id(),
                self.pubkey,
                self.mint_pubkey,
                authority.pubkey(),
                authority.pubkey(),
                self.metadata_pubkey,
                authority.pubkey(),
                max_supply,
            )],
            Some(&authority.pubkey()),
            &[authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }
}
