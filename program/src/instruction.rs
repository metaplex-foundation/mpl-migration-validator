use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::state::UnlockMethod;

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
    pub collection_size: Option<u32>,
    pub new_update_authority: Option<Pubkey>,
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

    #[account(0, writable, signer, name="authority", desc = "The collection authority")]
    #[account(1, writable, name="migration_state", desc = "The migration state account")]
    Update(UpdateArgs),

    /// Permissionless handler to initialize the program signer
    #[account(0, writable, signer, name="payer", desc="Paying account for initiate migration")]
    #[account(1, writable, name="program_signer", desc="Program signer account")]
    #[account(2, name="system_program", desc = "System program")]
    InitSigner,

    /// Start a migration if it is eligible.
    #[account(0, writable, signer, name="payer", desc="Paying account for initiate migration")]
    #[account(1, signer, name="authority", desc = "The collection authority")]
    #[account(2, name="collection_mint", desc = "The mint account of the collection parent NFT")]
    #[account(3, name="collection_metadata", desc = "The metadata account of the collection parent NFT")]
    #[account(4, name="program_signer", desc="Program signer PDA")]
    #[account(5, writable, name="delegate_record", desc = "The collection delegate record of for the program signer and the collection")]
    #[account(6, writable, name="migration_state", desc = "The migration state account")]
    #[account(7, name="spl_token_program", desc="Token Program")]
    #[account(8, name="system_program", desc = "System program")]
    #[account(9, name="token_metadata_program", desc = "Token Metadata program for the CPI call")]
    Start,

    /// Migrate an asset.    
    #[account(0, writable, name="item_metadata", desc="Metadata account")]
    #[account(1, writable, name="item_edition", desc="Edition account")]
    #[account(2, writable, name="item_token", desc="Token account")]
    #[account(3, name="token_owner", desc="Token owner")]
    #[account(4, name="token_owner_program,", desc="Program that owns the token owner")]
    #[account(5, name="token_owner_program_buffer,", desc="Executable buffer account of the program owner")]
    #[account(6, name="item_mint", desc="Mint account")]
    #[account(7, writable, signer, name="payer", desc="Pays for migration costs")]
    #[account(8, name="program_signer", desc="Program signer PDA")]
    #[account(9, name="collection_metadata", desc="Collection metadata account")]
    #[account(10, name="delegate_record", desc="Update authority or delegate")]
    #[account(11, writable, name="token_record", desc="Update authority or delegate")]
    #[account(12, name="system_program", desc="System program")]
    #[account(13, name="sysvar_instructions", desc="Instruction sysvar account")]
    #[account(14, name="spl_token_program", desc="Token Program")]
    #[account(15, optional, name="authorization_rules_program", desc="Token Authorization Rules Program")]
    #[account(16, optional, name="authorization_rules", desc="Token Authorization Rules account")]
    #[account(17, writable, name="migration_state", desc = "The migration state account")]
    #[account(18, name="token_metadata_program", desc = "Token Metadata program for the CPI call")]
    #[default_optional_accounts]
    Migrate,
}

// pub fn update(authority: Pubkey, migration_state: Pubkey, args: UpdateArgs) -> Instruction {
//     let data = MigrationInstruction::Update(args).try_to_vec().unwrap();
//     Instruction {
//         program_id: crate::ID,
//         accounts: vec![
//             AccountMeta::new(authority, true),
//             AccountMeta::new(migration_state, false),
//         ],
//         data,
//     }
// }

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

// pub fn init_signer(payer: Pubkey) -> Instruction {
//     let data = MigrationInstruction::InitSigner.try_to_vec().unwrap();
//     Instruction {
//         program_id: crate::ID,
//         accounts: vec![
//             AccountMeta::new(payer, true),
//             AccountMeta::new(PROGRAM_SIGNER, false),
//             AccountMeta::new_readonly(solana_program::system_program::ID, false),
//         ],
//         data,
//     }
// }
