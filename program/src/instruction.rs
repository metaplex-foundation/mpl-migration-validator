use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::state::{UnlockMethod, SPL_TOKEN_ID};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitializeArgs {
    pub rule_set: Option<Pubkey>,
    pub unlock_method: UnlockMethod,
    pub collection_size: u32,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct UpdateArgs {
    pub rule_set: Option<Pubkey>,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum MigrationInstruction {
    /// Initiate a migration, creating the migration state and starting the countdown.
    #[account(0, writable, signer, name="payer", desc="Paying account for initiate migration")]
    #[account(1, signer, name="authority", desc = "The collection authority")]
    #[account(2, name="collection_mint", desc = "The mint account of the collection parent NFT")]
    #[account(3, name="collection_metadata", desc = "The metadata account of the collection parent NFT")]
    #[account(4, writable, name="migration_state", desc = "The migration state account")]
    #[account(5, name="system_program", desc = "System program")]
    Initialize(InitializeArgs),

    /// Close a migration state account, if the migration is not in progress.
    #[account(0, writable, signer, name="authority", desc="The collection authority")]
    #[account(1, writable, name="migration_state", desc = "The migration state account")]
    #[account(2, name="system_program", desc = "System program")]
    Close,

    #[account(0, signer, name="authority", desc = "The collection authority")]
    #[account(1, writable, name="migration_state", desc = "The migration state account")]
    #[account(2, optional, name="vote_account", desc = "SPL governance vote account")]
    Update(UpdateArgs),

    /// Start a migration if it is eligible.
    #[account(0, writable, signer, name="payer", desc="Paying account for initiate migration")]
    #[account(1, signer, name="authority", desc = "The collection authority")]
    #[account(2, name="collection_mint", desc = "The mint account of the collection parent NFT")]
    #[account(3, name="collection_metadata", desc = "The metadata account of the collection parent NFT")]
    #[account(4, name="delegate", desc = "The collection delegate. This should be the program signer.")]
    #[account(5, name="delegate_record", desc = "The collection delegate record of for the program signer and the collection")]
    #[account(6, writable, name="migration_state", desc = "The migration state account")]
    #[account(7, name="spl_token_program", desc="Token Program")]
    #[account(8, name="system_program", desc = "System program")]
    Start,

    /// Migrate an asset.    
    #[account(0, writable, signer, name="payer", desc="Pays for migration costs")]
    #[account(1, writable, name="metadata", desc="Metadata account")]
    #[account(2, name="edition", desc="Edition account")]
    #[account(3, writable, name="token", desc="Token account")]
    #[account(4, name="mint", desc="Mint account")]
    #[account(5, name="delegate_record", desc="Update authority or delegate")]
    #[account(6, name="collection_metadata", desc="Collection metadata account")]
    #[account(7, writable, name="migration_state", desc = "The migration state account")]
    #[account(8, name="program_signer", desc="Program signer PDA")]
    #[account(9, name="system_program", desc="System program")]
    #[account(10, name="sysvar_instructions", desc="Instruction sysvar account")]
    #[account(11, name="spl_token_program", desc="Token Program")]
    #[account(12, name="token_metadata_program", desc = "Token Metadata program for the CPI call")]
    #[account(13, optional, name="authorization_rules_program", desc="Token Authorization Rules Program")]
    #[account(14, optional, name="authorization_rules", desc="Token Authorization Rules account")]
    #[default_optional_accounts]
    Migrate,

    /// Permissionless handler to initialize the program signer
    /// 
    #[account(0, writable, signer, name="payer", desc="Paying account for initiate migration")]
    #[account(1, writable, name="program_signer", desc="Program signer account")]
    #[account(2, name="system_program", desc = "System program")]
    InitSigner,
}

pub fn initialize(
    payer: Pubkey,
    authority: Pubkey,
    collection_mint: Pubkey,
    collection_metadata: Pubkey,
    migration_state: Pubkey,
    args: InitializeArgs,
) -> Instruction {
    let data = MigrationInstruction::Initialize(args).try_to_vec().unwrap();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(collection_mint, false),
            AccountMeta::new_readonly(collection_metadata, false),
            AccountMeta::new(migration_state, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
        ],
        data,
    }
}

pub fn update(
    authority: Pubkey,
    migration_state: Pubkey,
    vote_account: Option<Pubkey>,
    args: UpdateArgs,
) -> Instruction {
    let data = MigrationInstruction::Update(args).try_to_vec().unwrap();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(migration_state, false),
            AccountMeta::new_readonly(vote_account.unwrap_or_default(), false),
        ],
        data,
    }
}

pub fn close(authority: Pubkey, migration_state: Pubkey) -> Instruction {
    let data = MigrationInstruction::Close.try_to_vec().unwrap();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(migration_state, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
        ],
        data,
    }
}

pub fn init_signer(payer: Pubkey, program_signer: Pubkey) -> Instruction {
    let data = MigrationInstruction::InitSigner.try_to_vec().unwrap();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(program_signer, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
        ],
        data,
    }
}

pub fn start(
    payer: Pubkey,
    authority: Pubkey,
    collection_mint: Pubkey,
    collection_metadata: Pubkey,
    delegate: Pubkey,
    delegate_record: Pubkey,
    migration_state: Pubkey,
) -> Instruction {
    let data = MigrationInstruction::Start.try_to_vec().unwrap();
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(collection_mint, false),
            AccountMeta::new_readonly(collection_metadata, false),
            AccountMeta::new_readonly(delegate, false),
            AccountMeta::new(delegate_record, false),
            AccountMeta::new(migration_state, false),
            AccountMeta::new_readonly(SPL_TOKEN_ID, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
        ],
        data,
    }
}
