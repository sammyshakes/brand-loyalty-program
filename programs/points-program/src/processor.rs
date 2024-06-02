use crate::error::ErrorCode;
use crate::{MintPoints, TransferPoints};
use anchor_lang::prelude::*;

pub fn mint_points(ctx: Context<MintPoints>, amount: u64) -> Result<()> {
    let mint = &mut ctx.accounts.points_mint;
    let user_token_account = &mut ctx.accounts.user_token_account;

    let cpi_accounts = anchor_spl::token::MintTo {
        mint: mint.to_account_info(),
        to: user_token_account.to_account_info(),
        authority: ctx.accounts.brand.to_account_info(),
    };

    let cpi_program = ctx.accounts.points_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    anchor_spl::token::mint_to(cpi_ctx, amount)?;

    Ok(())
}

pub fn transfer_points(ctx: Context<TransferPoints>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from;
    let to = &mut ctx.accounts.to;

    if from.balance < amount {
        return Err(ErrorCode::InsufficientFunds.into());
    }

    from.balance -= amount;
    to.balance += amount;
    Ok(())
}
