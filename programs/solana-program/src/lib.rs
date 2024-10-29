use anchor_lang::prelude::*;

pub mod instructions;
pub mod errors;
pub mod state;
pub mod consts;

use instructions::*;


declare_id!("SBGGtnX4A8nX3zGrtjxXp8VZkk5hEpyoamwcjpNLnaz");

#[program]
pub mod solana_program {
    use state::ShortingConfirguration;

    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, token_name, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        mint::mint_token(ctx)
    }

    pub fn initialize_curve(ctx: Context<InitializeCurveConfiguration>, fees: f64) -> Result<()> {
        initialize::initialize_curve(ctx, fees)
    }

    pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        create_pool::create_pool(ctx)
    }

    pub fn create_short_pool(ctx: Context<CreateShortingPool>) -> Result<()> {
        create_short_pool::create_short_pool(ctx)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        buy::buy(ctx, amount)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
        sell::sell(ctx, amount, bump)
    }

    pub fn initialize_shortpool(ctx: Context<InitializeShortpoolConfiguration>, collateral_leverage: f64, hourly_borrow_rate: f64) -> Result<()> {
        initialize_shortpool::initialize_shortpool(ctx, collateral_leverage, hourly_borrow_rate)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        borrow::borrow(ctx, amount)
    }

    pub fn refund(ctx: Context<Refund>, amount: u64, sol_amount: u64, bump: u8) -> Result<()> {
        refund::refund(ctx, amount, sol_amount, bump)
    }
}

