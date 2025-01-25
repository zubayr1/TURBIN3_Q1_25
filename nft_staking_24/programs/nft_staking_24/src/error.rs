use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("User has already staked the maximum amount")]
    MaxStakeReached,
}
