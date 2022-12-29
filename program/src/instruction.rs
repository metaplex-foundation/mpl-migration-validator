use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitiateArgs {
    pub rule_set: Pubkey,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum MigrateInstruction {
    /// Initiate a migration, creating the migration state and starting the countdown.
    #[account(0, writable, signer, name="payer", desc="Paying account for initiate migration")]
    #[account(1, signer, name="authority", desc = "The collection authority")]
    #[account(2, name="collection_mint", desc = "The mint account of the collection parent NFT")]
    #[account(3, name="collection_metadata", desc = "The metadata account of the collection parent NFT")]
    #[account(4, writable, name="migration_state", desc = "The migration state account")]
    #[account(5, name="system_program", desc = "System program")]
    Initiate(InitiateArgs),

    /// Description of this instruction
    #[account(0, writable, signer, name="signed_writable_account", desc="signed, writable account description")]
    #[account(1, writable, name="writable_account", desc = "writable, non signed account description")]
    #[account(2, name="non_writable_account", desc = "non signed, non writable account description")]
    #[account(3, name="token_program", desc = "Token program")]
    #[account(4, name="rent", desc = "Rent sysvar")]
    Cancel,

    /// Description of this instruction
    #[account(0, writable, signer, name="signed_writable_account", desc="signed, writable account description")]
    #[account(1, writable, name="writable_account", desc = "writable, non signed account description")]
    #[account(2, name="non_writable_account", desc = "non signed, non writable account description")]
    #[account(3, name="token_program", desc = "Token program")]
    #[account(4, name="rent", desc = "Rent sysvar")]
    Start,

    /// Description of this instruction
    #[account(0, writable, signer, name="signed_writable_account", desc="signed, writable account description")]
    #[account(1, writable, name="writable_account", desc = "writable, non signed account description")]
    #[account(2, name="non_writable_account", desc = "non signed, non writable account description")]
    #[account(3, name="token_program", desc = "Token program")]
    #[account(4, name="rent", desc = "Rent sysvar")]
    Migrate,
    
}

pub fn initiate(
    payer: Pubkey,
    authority: Pubkey,
    collection_mint: Pubkey,
    collection_metadata: Pubkey,
    migration_state: Pubkey,
    rule_set: Pubkey,
) -> Instruction {
    let args = InitiateArgs { rule_set };
    let data = MigrateInstruction::Initiate(args).try_to_vec().unwrap();
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
