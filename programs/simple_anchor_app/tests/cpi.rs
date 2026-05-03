use mollusk_svm::{program::create_program_account_loader_v3, result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
};

const SIMPLE_ANCHOR_APP_ID: Pubkey = pubkey!("5UDda9Uq56F75arfkrLX2UHy7EbXtW4DQj3B8HSgn7a2");
const CPI_TARGET_ID: Pubkey = pubkey!("HtH3m4682j9Dq9bGx7K41fW7nT3PUTWi3dHbLUHY7ZYX");

// sha256("global:initialize")[..8]
const INITIALIZE_DISC: [u8; 8] = [0xaf, 0xaf, 0x6d, 0x1f, 0x0d, 0x98, 0x9b, 0xed];

#[test]
fn make_a_cpi() {
    // std::env::set_var(
    //     "SBF_OUT_DIR",
    //     concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/deploy"),
    // );

    let mut mollusk = Mollusk::new(&SIMPLE_ANCHOR_APP_ID, "simple_anchor_app");
    mollusk.add_program(&CPI_TARGET_ID, "cpi_target");

    let payer = Pubkey::new_unique();
    let payer_account = Account {
        lamports: 1_000_000_000,
        ..Account::default()
    };

    let mut data = Vec::with_capacity(40);
    data.extend_from_slice(&INITIALIZE_DISC);
    data.extend_from_slice(&CPI_TARGET_ID.to_bytes());

    let instruction = Instruction {
        program_id: SIMPLE_ANCHOR_APP_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(CPI_TARGET_ID, false),
        ],
        data,
    };

    let accounts = vec![
        (payer, payer_account),
        (
            CPI_TARGET_ID,
            create_program_account_loader_v3(&CPI_TARGET_ID),
        ),
    ];

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}
