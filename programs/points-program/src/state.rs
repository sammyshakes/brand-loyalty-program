use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub admin: Pubkey,
}

#[account]
pub struct Mint {
    pub supply: u64,
}

#[account]
pub struct TokenAccount {
    pub balance: u64,
    pub owner: Pubkey,
}
