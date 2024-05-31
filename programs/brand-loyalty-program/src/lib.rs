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
        let brand = &mut ctx.accounts.brand;
        brand.name = brand_name;
        brand.owner = *ctx.accounts.admin.key;

        // Derive the PDA for the brand's points mint
        let (points_mint_pda, _bump_seed) =
            Pubkey::find_program_address(&[b"points_mint", brand.key().as_ref()], ctx.program_id);

        // Ensure the derived PDA is stored in the brand's state
        brand.points_mint = points_mint_pda;

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
    #[account(init, payer = admin, space = 8 + 32 + 40)] // Adjust space as needed
    pub brand: Account<'info, state::Brand>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
