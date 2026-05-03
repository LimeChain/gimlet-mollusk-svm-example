#![no_std]

use pinocchio::{error::ProgramError, AccountView, Address, ProgramResult};
use solana_program_log::log;

pub const ID: Address =
    Address::from_str_const("HtH3m4682j9Dq9bGx7K41fW7nT3PUTWi3dHbLUHY7ZYX");

// Anchor-compatible 8-byte discriminator: sha256("global:ping")[..8].
pub const PING_IX: [u8; 8] = [0xad, 0x00, 0x5e, 0xec, 0x49, 0x85, 0xe1, 0x99];

// `entrypoint!` would pull in `default_panic_handler!`, which requires `std`
// in the dependency tree. Since this crate (and all its deps) are `no_std`, we
// assemble the entrypoint from the three sub-macros and use the `no_std`
// panic handler directly.
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::program_entrypoint!(process_instruction);
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::default_allocator!();
#[cfg(not(feature = "no-entrypoint"))]
pinocchio::nostd_panic_handler!();

pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let disc: [u8; 8] = instruction_data[..8].try_into().unwrap();

    match disc {
        PING_IX => ping(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn ping(accounts: &[AccountView]) -> ProgramResult {
    let payer = accounts
        .first()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !payer.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }

    log!("Ping received by cpi_target!");
    Ok(())
}
