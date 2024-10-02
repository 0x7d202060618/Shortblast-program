use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub fn create_short_pool(ctx: Context<CreateShortingPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.set_inner(ShortPool::new(
        ctx.accounts.payer.key(),
        ctx.accounts.token_mint.key(),
        ctx.bumps.pool,
    ));
    
    let token_accounts = (
        &mut *ctx.accounts.token_mint,
        &mut *ctx.accounts.pool_token_account,
        &mut *ctx.accounts.user_token_account,
    );

    pool.add_liquidity(
        token_accounts, 
 88_000_000_000_000_000,
        &mut ctx.accounts.pool_sol_vault, 
        &ctx.accounts.payer, 
        &ctx.accounts.token_program,
        &ctx.accounts.system_program
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateShortingPool<'info> {
    #[account(
        init,
        space = ShortPool::ACCOUNT_SIZE,
        payer = payer,
        seeds = [ShortPool::POOL_SEED_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool: Box<Account<'info, ShortPool>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = pool
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [ShortPool::SOL_VAULT_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
