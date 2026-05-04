use mollusk_svm::{program::create_program_account_loader_v3, result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
};

const PRIMARY_ID: Pubkey = pubkey!("5UDda9Uq56F75arfkrLX2UHy7EbXtW4DQj3B8HSgn7a2");
const CPI_TARGET_ID: Pubkey = pubkey!("HtH3m4682j9Dq9bGx7K41fW7nT3PUTWi3dHbLUHY7ZYX");

#[test]
fn make_a_cpi() {
    let mut mollusk = Mollusk::new_debuggable(&PRIMARY_ID, "primary", true);
    mollusk.add_program(&CPI_TARGET_ID, "cpi_target");

    let payer = Pubkey::new_unique();
    let payer_account = Account {
        lamports: 1_000_000_000,
        ..Account::default()
    };

    let instruction = Instruction {
        program_id: PRIMARY_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(CPI_TARGET_ID, false),
        ],
        data: CPI_TARGET_ID.to_bytes().to_vec(),
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
