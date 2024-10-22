use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.set_inner(LiquidityPool::new(
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
 800_000_000_000_000_000,
        &mut ctx.accounts.pool_sol_vault, 
        &ctx.accounts.payer, 
        &ctx.accounts.token_program,
        &ctx.accounts.system_program
    )?;

    emit!(CreatePoolEvent {
        address: pool.key(),
        creator: ctx.accounts.payer.key(),    
        token_mint: ctx.accounts.token_mint.key(),      
        total_supply: pool.total_supply,  
        reserve_token: pool.reserve_token,
        reserve_sol: pool.reserve_sol,   
        bump: pool.bump,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    #[account(
        init,
        space = LiquidityPool::ACCOUNT_SIZE,
        payer = payer,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

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
        seeds = [LiquidityPool::SOL_VAULT_PREFIX.as_bytes(), token_mint.key().as_ref()],
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



#[event]  
pub struct CreatePoolEvent {
    pub address: Pubkey,
    pub creator: Pubkey,    // Public key of the pool creator
    pub token_mint: Pubkey,      // Public key of the token mint in the liquidity pool
    pub total_supply: u64,  // Total supply of liquidity tokens
    pub reserve_token: u64, // Reserve amount of token in the pool
    pub reserve_sol: u64,   // Reserve amount of sol_token in the pool
    pub bump: u8,           // Nonce for the program-derived address
}  
