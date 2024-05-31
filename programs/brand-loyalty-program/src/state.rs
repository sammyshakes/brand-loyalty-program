use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub admin: Pubkey,
}

#[account]
pub struct Brand {
    pub name: String,
    pub owner: Pubkey,
}