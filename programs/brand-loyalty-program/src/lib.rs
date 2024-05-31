use anchor_lang::prelude::*;

pub mod error;
pub mod processor;
pub mod state;

declare_id!("AT6whjLqybw5CzMA7g5Lnj6mXcF4phGV34e9MMzQtc5");

#[program]
mod brand_loyalty_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = admin;
        Ok(())
    }

    pub fn create_brand(ctx: Context<CreateBrand>, brand_name: String) -> Result<()> {
        require!(
            ctx.accounts.state.admin == *ctx.accounts.admin.key,
            error::ErrorCode::Unauthorized
        );
        processor::create_brand(ctx, brand_name)
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
pub struct CreateBrand<'info> {
    #[account(mut)]
    pub state: Account<'info, state::State>,
    #[account(init, payer = admin, space = 8 + 32 + 40)] // adjust space as needed
    pub brand: Account<'info, state::Brand>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
