use anchor_lang::prelude::*;

pub mod instructions;
pub mod errors;
pub mod state;
pub mod consts;

use instructions::*;


declare_id!("HiJRFQoQ6pZKtNvUaNeQkT6xpex7EWyzKVtXbJgDzNEq");

#[program]
pub mod solana_program {
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

    // pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    //     buy::buy(ctx, amount)
    // }

    // pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
    //     sell::sell(ctx, amount, bump)
    // }
}

