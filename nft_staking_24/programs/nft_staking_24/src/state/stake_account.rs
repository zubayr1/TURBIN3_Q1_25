use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub bump: u8,
    pub user: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64,
}
