use mpl_token_auth_rules::{
    instruction::{builders::CreateOrUpdateBuilder, CreateOrUpdateArgs, InstructionBuilder},
    payload::Payload,
    state::{CompareOp, Rule, RuleSet},
};
use mpl_token_metadata::{
    processor::{AuthorizationData, TransferScenario},
    state::{Operation, PayloadKey},
};
use rmp_serde::Serializer;
use serde::Serialize;
use solana_program::system_program;
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

static PROGRAM_ALLOW_LIST: [Pubkey; 2] = [mpl_token_auth_rules::ID, rooster::ID];

macro_rules! get_primitive_rules {
    (
        $source_is_sys_prog_owned_wallet:ident,
        $dest_program_allow_list:ident,
        $dest_pda_match:ident,
        $source_program_allow_list:ident,
        $source_pda_match:ident,
        $dest_is_sys_prog_owned_wallet:ident,
        $nft_amount:ident,
    ) => {
        let $source_is_sys_prog_owned_wallet = Rule::ProgramOwned {
            program: system_program::ID,
            field: PayloadKey::Source.to_string(),
        };

        let $dest_program_allow_list = Rule::ProgramOwnedList {
            programs: PROGRAM_ALLOW_LIST.to_vec(),
            field: PayloadKey::Destination.to_string(),
        };

        let $dest_pda_match = Rule::PDAMatch {
            program: None,
            pda_field: PayloadKey::Destination.to_string(),
            seeds_field: PayloadKey::DestinationSeeds.to_string(),
        };

        let $source_program_allow_list = Rule::ProgramOwnedList {
            programs: PROGRAM_ALLOW_LIST.to_vec(),
            field: PayloadKey::Source.to_string(),
        };

        let $source_pda_match = Rule::PDAMatch {
            program: None,
            pda_field: PayloadKey::Source.to_string(),
            seeds_field: PayloadKey::SourceSeeds.to_string(),
        };

        let $dest_is_sys_prog_owned_wallet = Rule::ProgramOwned {
            program: system_program::ID,
            field: PayloadKey::Destination.to_string(),
        };
        let $nft_amount = Rule::Amount {
            field: PayloadKey::Amount.to_string(),
            amount: 1,
            operator: CompareOp::Eq,
        };
    };
}

pub async fn create_default_metaplex_rule_set(
    context: &mut ProgramTestContext,
    creator: Keypair,
) -> (Pubkey, AuthorizationData) {
    let name = String::from("Metaplex Royalty Enforcement");
    let (ruleset_addr, _ruleset_bump) =
        mpl_token_auth_rules::pda::find_rule_set_address(creator.pubkey(), name.clone());

    get_primitive_rules!(
        source_is_sys_prog_owned_wallet,
        dest_program_allow_list,
        dest_pda_match,
        source_program_allow_list,
        source_pda_match,
        dest_is_sys_prog_owned_wallet,
        nft_amount,
    );

    // amount is 1 &&
    // (source is on allow list && source is a PDA) ||
    // (dest is on allow list && dest is a PDA) ||
    // (source is a sys prog owned wallet && dest is a sys prog owned wallet)
    let transfer_rule = Rule::All {
        rules: vec![
            nft_amount,
            Rule::Any {
                rules: vec![
                    Rule::All {
                        rules: vec![source_program_allow_list, source_pda_match],
                    },
                    Rule::All {
                        rules: vec![dest_program_allow_list, dest_pda_match],
                    },
                    Rule::All {
                        rules: vec![
                            source_is_sys_prog_owned_wallet,
                            dest_is_sys_prog_owned_wallet,
                        ],
                    },
                ],
            },
        ],
    };

    let owner_operation = Operation::Transfer {
        scenario: TransferScenario::Holder,
    };

    let transfer_delegate_operation = Operation::Transfer {
        scenario: TransferScenario::TransferDelegate,
    };

    let sale_delegate_operation = Operation::Transfer {
        scenario: TransferScenario::SaleDelegate,
    };

    let mut royalty_rule_set = RuleSet::new(name, creator.pubkey());
    royalty_rule_set
        .add(owner_operation.to_string(), transfer_rule.clone())
        .unwrap();
    royalty_rule_set
        .add(
            transfer_delegate_operation.to_string(),
            transfer_rule.clone(),
        )
        .unwrap();
    royalty_rule_set
        .add(sale_delegate_operation.to_string(), transfer_rule.clone())
        .unwrap();

    // Serialize the RuleSet using RMP serde.
    let mut serialized_data = Vec::new();
    royalty_rule_set
        .serialize(&mut Serializer::new(&mut serialized_data))
        .unwrap();

    // Create a `create` instruction.
    let create_ix = CreateOrUpdateBuilder::new()
        .rule_set_pda(ruleset_addr)
        .payer(creator.pubkey())
        .build(CreateOrUpdateArgs::V1 {
            serialized_rule_set: serialized_data,
        })
        .unwrap()
        .instruction();

    // Add it to a transaction.
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&creator.pubkey()),
        &[&creator],
        context.last_blockhash,
    );

    // Process the transaction.
    context
        .banks_client
        .process_transaction(create_tx)
        .await
        .expect("creation should succeed");

    // Client can add additional rules to the Payload but does not need to in this case.
    let payload = Payload::new();
    let auth_data = AuthorizationData { payload };

    (ruleset_addr, auth_data)
}
