use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Freeze Period Not Over")]
    FreezePeriodNotOver,

    #[msg("Max Stake Reached")]
    MaxStakeReached,
}
