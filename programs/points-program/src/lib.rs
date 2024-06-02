use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount};

pub mod error;
pub mod processor;
pub mod state;

declare_id!("3ea2G9pANfzABML6v35rEH2h4AVuR6c5jfRaC684w3br");

#[program]
mod points_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = admin;
        Ok(())
    }

    pub fn mint_points(ctx: Context<MintPoints>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.points_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.brand.to_account_info(),
        };

        let cpi_program = ctx.accounts.points_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn transfer_points(ctx: Context<TransferPoints>, amount: u64) -> Result<()> {
        // Ensure the transfer is authorized by the admin or a derived PDA
        require!(
            ctx.accounts.state.admin == *ctx.accounts.admin.key,
            error::ErrorCode::Unauthorized
        );
        processor::transfer_points(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 32)]
    pub state: Account<'info, state::State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintPoints<'info> {
    #[account(mut)]
    pub state: Account<'info, state::State>,
    #[account(mut)]
    pub brand: Account<'info, state::Brand>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub points_mint: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is not dangerous because we only use this to call the token program
    pub points_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferPoints<'info> {
    #[account(mut)]
    pub state: Account<'info, state::State>,
    #[account(mut)]
    pub from: Account<'info, state::TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, state::TokenAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
}
