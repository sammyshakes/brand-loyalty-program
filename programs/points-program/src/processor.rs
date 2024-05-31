use crate::error::ErrorCode;
use crate::{MintPoints, TransferPoints};
use anchor_lang::prelude::*;

pub fn mint_points(ctx: Context<MintPoints>, amount: u64) -> Result<()> {
    let mint = &mut ctx.accounts.mint;
    mint.supply += amount;
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
