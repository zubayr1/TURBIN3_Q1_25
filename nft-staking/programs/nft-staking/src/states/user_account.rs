use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u64,
    pub amount_stake: u8,
    pub bump: u8,
}
