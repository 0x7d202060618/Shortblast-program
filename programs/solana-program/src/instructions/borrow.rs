use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{LiquidityPool, ShortPool, ShortPoolAccount, ShortingConfirguration};

pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let token_one_accounts = (
        &mut *ctx.accounts.token_mint,
        &mut *ctx.accounts.pool_token_account,
        &mut *ctx.accounts.user_token_account,
    );

    pool.borrow(
        &ctx.accounts.shorting_configuration_account,
        &ctx.accounts.token_pool,
        token_one_accounts,
        &mut ctx.accounts.pool_sol_vault,
        amount,
        &ctx.accounts.user,
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
        ctx.accounts.user.key()
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(
        mut,
        seeds = [ShortingConfirguration::SEED.as_bytes()],
        bump,
    )]
    pub shorting_configuration_account: Box<Account<'info, ShortingConfirguration>>,

    #[account(
        mut,
        seeds = [ShortPool::POOL_SEED_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, ShortPool>>,

    #[account(
        mut,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump = token_pool.bump
    )]
    pub token_pool: Box<Account<'info, LiquidityPool>>,

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
        init_if_needed,
        payer = user,
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

