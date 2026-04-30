use anchor_lang::prelude::*;

declare_id!("FfC7KTHPQ2DxA2dD5aTaLn7vcyA1dF9HW3gSMxWPYibZ");

#[program]
pub mod cpi_target {
    use super::*;

    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        msg!("Ping received by cpi_target!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ping<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}
