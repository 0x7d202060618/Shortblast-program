use crate::consts::INITIAL_EXPONENT;
use crate::consts::INITIAL_LAMPORTS_FOR_POOL;
use crate::consts::INITIAL_PROPORTION;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::errors::CustomError;

use super::LiquidityPool;



#[account]
pub struct ShortingConfirguration {
    pub collateral_leverage: f64,
    pub hourly_borrow_rate: f64
}


impl ShortingConfirguration {
    pub const SEED: &'static str = "ShortConfiguration";

    // Discriminator (8) + f64 (8) + f64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 8 + 8;

    pub fn new(collateral_leverage: f64, hourly_borrow_rate: f64 ) -> Self {
        Self { collateral_leverage, hourly_borrow_rate }
    }
}



#[account]
pub struct ShortPool {
    pub creator: Pubkey,    // Public key of the pool creator
    pub token: Pubkey,      // Public key of the token mint in the liquidity pool
    pub total_supply: u64,  // Total supply of liquidity tokens
    pub reserve_token: u64, // Reserve amount of token in the pool
    pub reserve_sol: u64,   // Reserve amount of sol_token in the pool
    pub bump: u8,           // Nonce for the program-derived address
    pub created_at:  i64,    //Pool Create timestamps
    // pub borrow_info: Vec<UserBorrow>
}

impl ShortPool {
    pub const POOL_SEED_PREFIX: &'static str = "shorting_pool";
    pub const SOL_VAULT_PREFIX: &'static str = "shorting_pool_sol_vault";

    // Discriminator (8) + Pubkey (32) + Pubkey (32) + totalsupply (8)
    // + reserve one (8) + reserve two (8) + Bump (1) + created_at(8)
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 8;

    // Constructor to initialize a ShortPool with two tokens and a bump for the PDA
    pub fn new(creator: Pubkey, token: Pubkey, bump: u8) -> Self {
        Self {
            creator,
            token,
            total_supply: 0_u64,
            reserve_token: 0_u64,
            reserve_sol: 0_u64,
            // borrow_info: vec![],
            bump,
            created_at: Clock::get().unwrap().unix_timestamp
        }
    }
}

pub trait ShortPoolAccount<'info> {
    // Updates the token reserves in the liquidity pool
    fn update_reserves(&mut self, reserve_token: u64, reserve_sol: u64) -> Result<()>;

    // Allows adding liquidity by depositing an amount of two tokens and getting back pool shares
    fn add_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        amount: u64,
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    // Allows removing liquidity by burning pool shares and receiving back a proportionate amount of tokens
    fn remove_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_account: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        bump: u8,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn borrow(
        &mut self,
        configuration_account: &Account<'info, ShortingConfirguration>,
        token_pool: &Account<'info, LiquidityPool>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
        user: Pubkey
    ) -> Result<()>;

    fn refund(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        bump: u8,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
        user: Pubkey
    ) -> Result<()>;

    fn transfer_token_from_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn transfer_token_to_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn transfer_sol_to_pool(
        &self,
        from: &Signer<'info>,
        to: &mut AccountInfo<'info>,
        amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn transfer_sol_from_pool(
        &self,
        from: &mut AccountInfo<'info>,
        to: &Signer<'info>,
        amount: u64,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()>;
}

impl<'info> ShortPoolAccount<'info> for Account<'info, ShortPool> {
    fn update_reserves(&mut self, reserve_token: u64, reserve_sol: u64) -> Result<()> {
        self.reserve_token = reserve_token;
        self.reserve_sol = reserve_sol;
        Ok(())
    }

    fn add_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        amount: u64,
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        self.transfer_token_to_pool(
            token_accounts.2,
            token_accounts.1,
            amount,
            authority,
            token_program,
        )?;

        self.total_supply = 800_000_000_000_000_000;
        self.update_reserves(amount, INITIAL_LAMPORTS_FOR_POOL)?;

        Ok(())
    }

    fn remove_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        bump: u8,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        self.transfer_token_from_pool(
            token_accounts.1,
            token_accounts.2,
            token_accounts.1.amount as u64,
            token_program,
        )?;
        // let amount = self.to_account_info().lamports() - self.get_lamports();
        let amount = pool_sol_vault.to_account_info().lamports() as u64;
        self.transfer_sol_from_pool(pool_sol_vault, authority, amount, bump, system_program)?;

        Ok(())
    }

    fn borrow(
        &mut self,
        configuration_account: &Account<'info, ShortingConfirguration>,
        token_pool: &Account<'info, LiquidityPool>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
        user: Pubkey
    ) -> Result<()> {
        if amount == 0 {
            return err!(CustomError::InvalidAmount);
        }

        if amount > self.reserve_token {
            return err!(CustomError::NotEnoughTokenInVault);
        }
        let current_token_supply = (token_pool.total_supply as f64 - token_pool.reserve_token as f64) / 1_000_000_000.0;
        let token_price = INITIAL_PROPORTION * f64::exp(INITIAL_EXPONENT * current_token_supply); //buying price per 10M tokens

        let sol_amount = ((amount as f64 * configuration_account.collateral_leverage / 100.0 / 10000000.0) * token_price as f64).floor() as u64;

        self.reserve_sol += sol_amount;
        self.reserve_token -= amount;

        self.transfer_sol_to_pool(authority, pool_sol_vault, sol_amount, system_program)?;

        self.transfer_token_from_pool(
            token_accounts.1,
            token_accounts.2,
            amount,
            token_program,
        )?;
     
        // let user_borrow_info = self.borrow_info.iter_mut().find(|d| d.user == user);  
        // match user_borrow_info {  
        //     Some(entry) => {  
        //         if entry.sol_collateral != 0 {
        //            return err!(CustomError::NeedToRefundLastBorrow)
        //         }
        //         entry.token_amount = amount;
        //         entry.sol_collateral = sol_amount;
        //     }  
        //     None => {  
        //         self.borrow_info.push(UserBorrow{
        //             user: user,
        //             token_amount: amount,
        //             sol_collateral: sol_amount
        //         });
        //     }  
        // }  

        Ok(())
    }

    fn refund(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        bump: u8,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
        user: Pubkey
    ) -> Result<()> {
        // if amount == 0 {
        //     return err!(CustomError::InvalidAmount);
        // }

        // if self.reserve_token < amount {
        //     return err!(CustomError::TokenAmountToSellTooBig);
        // }

        // if self.reserve_sol < sol_amount {
        //     return err!(CustomError::NotEnoughSolInVault);
        // }

        // let user_borrow_info = self.borrow_info.iter_mut().find(|d| d.user == user);  
        // match user_borrow_info {  
        //     Some(entry) => {  
        //         let amount = entry.token_amount;
        //         let sol_amount = entry.sol_collateral;

        //         self.transfer_token_to_pool(
        //             token_accounts.2,
        //             token_accounts.1,
        //             amount as u64,
        //             authority,
        //             token_program,
        //         )?;

        //         self.transfer_sol_from_pool(pool_sol_vault, authority, sol_amount, bump, system_program)?;

        //         self.reserve_token += amount;
        //         self.reserve_sol -= sol_amount;

        //     }  
        //     None => {  
        //         return err!(CustomError::NoUserBorrow)
        //     }  
        // }  
     
        Ok(())
    }


    fn transfer_token_from_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: self.to_account_info(),
                },
                &[&[
                    ShortPool::POOL_SEED_PREFIX.as_bytes(),
                    self.token.key().as_ref(),
                    &[self.bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_token_to_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_sol_from_pool(
        &self,
        from: &mut AccountInfo<'info>,
        to: &Signer<'info>,
        amount: u64,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        // let pool_account_info = self.to_account_info();

        system_program::transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info().clone(),
                    to: to.to_account_info().clone(),
                },
                &[&[
                    ShortPool::SOL_VAULT_PREFIX.as_bytes(),
                    self.token.key().as_ref(),
                    &[bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_sol_to_pool(
        &self,
        from: &Signer<'info>,
        to: &mut AccountInfo<'info>,
        amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        // let pool_account_info = self.to_account_info();

        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy)]
pub struct UserBorrow {
    pub user: Pubkey, // immutable
    pub token_amount : u64,
    pub sol_collateral: u64
}