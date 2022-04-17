use anchor_lang::prelude::*;

#[account]
pub struct UserLedger {
    pub owner: Pubkey,
    pub total_yi_underlying_deposit: u64,
}

impl UserLedger {
    pub const LEN: usize =
        8 + // discriminator
        32 + // owner
        8 // total_yi_underlying_deposit
        ;
}