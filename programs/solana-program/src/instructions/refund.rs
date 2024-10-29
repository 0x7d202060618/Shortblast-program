use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{ShortPool, ShortPoolAccount};

pub fn refund(ctx: Context<Refund>, amount: u64, sol_amount: u64, bump: u8) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let token_one_accounts: (&mut Account<'_, Mint>, &mut Account<'_, TokenAccount>, &mut Account<'_, TokenAccount>) = (
        &mut *ctx.accounts.token_mint,
        &mut *ctx.accounts.pool_token_account,
        &mut *ctx.accounts.user_token_account,
    );

    pool.refund(
        token_one_accounts,
        &mut ctx.accounts.pool_sol_vault,
        bump,
        &ctx.accounts.user,
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
        ctx.accounts.user.key()
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        seeds = [ShortPool::POOL_SEED_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, ShortPool>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = pool
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [ShortPool::SOL_VAULT_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
