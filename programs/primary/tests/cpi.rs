use mollusk_svm::{program::create_program_account_loader_v3, result::Check, Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[test]
fn make_a_cpi() {
    let primary_id = Pubkey::new_unique();
    let cpi_target_id = Pubkey::new_unique();

    let mut mollusk = Mollusk::new_debuggable(&primary_id, "primary", true);
    mollusk.add_program(&cpi_target_id, "cpi_target");

    let payer = Pubkey::new_unique();
    let payer_account = Account {
        lamports: 1_000_000_000,
        ..Account::default()
    };

    let instruction = Instruction {
        program_id: primary_id,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(cpi_target_id, false),
        ],
        data: cpi_target_id.to_bytes().to_vec(),
    };

    let accounts = vec![
        (payer, payer_account),
        (
            cpi_target_id,
            create_program_account_loader_v3(&cpi_target_id),
        ),
    ];

    mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
}
