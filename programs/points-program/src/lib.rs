use anchor_lang::prelude::*;

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
        require!(
            ctx.accounts.state.admin == *ctx.accounts.admin.key,
            error::ErrorCode::Unauthorized
        );
        processor::mint_points(ctx, amount)
    }

    pub fn transfer_points(ctx: Context<TransferPoints>, amount: u64) -> Result<()> {
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
    pub mint: Account<'info, state::Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
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
