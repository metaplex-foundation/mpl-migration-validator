use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::state::MigrationType;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitializeArgs {
    pub rule_set: Option<Pubkey>,
    pub migration_type: MigrationType,
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
    #[account(0, signer, name="authority", desc = "The collection authority")]
    #[account(1, name="collection_mint", desc = "The mint account of the collection parent NFT")]
    #[account(2, name="collection_metadata", desc = "The metadata account of the collection parent NFT")]
    #[account(3, writable, name="migration_state", desc = "The migration state account")]
    #[account(4, name="system_program", desc = "System program")]
    Start,

    /// Description of this instruction
    #[account(0, writable, signer, name="signed_writable_account", desc="signed, writable account description")]
    #[account(1, writable, name="writable_account", desc = "writable, non signed account description")]
    #[account(2, name="non_writable_account", desc = "non signed, non writable account description")]
    #[account(3, name="token_program", desc = "Token program")]
    #[account(4, name="rent", desc = "Rent sysvar")]
    Migrate,
    
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
