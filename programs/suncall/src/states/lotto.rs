use anchor_lang::prelude::*;

#[account]
pub struct Lotto {
    /// Owner of the lotto
    pub owner: Pubkey,
    /// Public key of LottoState
    pub state: Pubkey,
    /// Yi underlying mint pubkey
    pub yi_underlying_mint: Pubkey,
    /// Yi mint pubkey
    pub yi_mint: Pubkey, 
}

impl Lotto {
    pub const LEN: usize =
        8 + // discriminator
        32 + // owner
        32 + // state
        32 + // yi_underlying_mint
        32 // yi_mint
        ;
}