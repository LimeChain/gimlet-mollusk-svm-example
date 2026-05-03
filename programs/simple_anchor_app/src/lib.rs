#![no_std]

use pinocchio::{
    cpi::invoke,
    error::ProgramError,
    instruction::{InstructionAccount, InstructionView},
    AccountView, Address, ProgramResult,
};
use solana_program_log::log;

pub const ID: Address =
    Address::from_str_const("5UDda9Uq56F75arfkrLX2UHy7EbXtW4DQj3B8HSgn7a2");

// Anchor-compatible 8-byte discriminator: sha256("global:initialize")[..8].
pub const INITIALIZE_IX: [u8; 8] = [0xaf, 0xaf, 0x6d, 0x1f, 0x0d, 0x98, 0x9b, 0xed];

// Discriminator forwarded over the CPI: sha256("global:ping")[..8].
// Kept inline (rather than importing `cpi_target::PING_IX`) so that workspace
// feature unification doesn't strip cpi_target's entrypoint when both
// programs are built together via `cargo build-sbf`.
const CPI_PING_IX: [u8; 8] = [0xad, 0x00, 0x5e, 0xec, 0x49, 0x85, 0xe1, 0x99];

#[cfg(not(feature = "no-entrypoint"))]
pinocchio::program_entrypoint!(process_instruction);
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::default_allocator!();
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::nostd_panic_handler!();

pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let disc: [u8; 8] = instruction_data[..8].try_into().unwrap();
    let rest = &instruction_data[8..];

    match disc {
        INITIALIZE_IX => initialize(program_id, accounts, rest),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initialize(
    _program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 32 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&data[..32]);
    let cpi_target_program_id = Address::new_from_array(bytes);

    let payer = accounts
        .first()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;
    let cpi_target_program = accounts
        .get(1)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !payer.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    log!("Greetings from simple_anchor_app");
    log!("Invoking cpi_target");

    if cpi_target_program.address() != &cpi_target_program_id {
        return Err(ProgramError::InvalidArgument);
    }

    let metas = [InstructionAccount::writable_signer(payer.address())];
    let ix = InstructionView {
        program_id: cpi_target_program.address(),
        accounts: &metas,
        data: &CPI_PING_IX,
    };
    invoke(&ix, &[payer])?;

    Ok(())
}
