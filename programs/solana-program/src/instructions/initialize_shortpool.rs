use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize_shortpool(
    ctx: Context<InitializeShortpoolConfiguration>,
    collateral_leverage: f64,
    hourly_borrow_rate: f64,
) -> Result<()> {
    let config = &mut ctx.accounts.configuration_account;

    config.set_inner(ShortingConfirguration::new(collateral_leverage, hourly_borrow_rate));

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeShortpoolConfiguration<'info> {
    #[account(
        init,
        space = ShortingConfirguration::ACCOUNT_SIZE,
        payer = admin,
        seeds = [ShortingConfirguration::SEED.as_bytes()],
        bump,
    )]
    pub configuration_account: Box<Account<'info, ShortingConfirguration>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
