use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount};

pub mod error;
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
        let brand = &mut ctx.accounts.brand;
        brand.name = brand_name;
        brand.owner = *ctx.accounts.admin.key;

        // Derive the PDA for the brand's points mint
        let (points_mint_pda, bump_seed) =
            Pubkey::find_program_address(&[b"points_mint", brand.key().as_ref()], ctx.program_id);

        // Store the derived PDA and bump seed in the brand's state
        brand.points_mint = points_mint_pda;
        brand.bump_seed = bump_seed;

        Ok(())
    }

    pub fn mint_points(ctx: Context<MintPoints>, amount: u64) -> Result<()> {
        let brand = &ctx.accounts.brand;
        let brand_key = brand.to_account_info().key;
        let bump_seed = brand.bump_seed;

        let seeds: &[&[u8]] = &[b"points_mint", brand_key.as_ref(), &[bump_seed]];
        let seeds_arr = [seeds]; // Bind to a variable with a longer lifetime

        // Create a CPI context to call the points program
        let cpi_accounts = MintTo {
            mint: ctx.accounts.points_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.brand.to_account_info(),
        };

        let cpi_program = ctx.accounts.points_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &seeds_arr);

        token::mint_to(cpi_ctx, amount)?;

        Ok(())
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
    #[account(init, payer = admin, space = 8 + 44 + 32 + 32)]
    pub brand: Account<'info, state::Brand>,
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
