use anchor_lang::prelude::*;

#[account]
pub struct LottoState {
    pub total_yi_underlying_deposit: u64,
    pub total_yi_deposit: u64,
    pub round: u64,
}

impl LottoState {
    pub const LEN: usize =
        8 + // discriminator
        8 + // total_underlying_deposit
        8 + // total_yi_deposit
        8 // round
        ;
}