use anchor_lang::prelude::*;

#[account]
pub struct Lotto {
    /// Owner of the lotto
    pub owner: Pubkey,
    /// Yi program pubkey
    pub yi_program: Pubkey,
    /// Yi token pubkey
    pub yi_token: Pubkey,
    /// Yi underlying mint pubkey
    pub yi_underlying_mint: Pubkey,
    /// Yi mint pubkey
    pub yi_mint: Pubkey, 
}

impl Lotto {
    pub const LEN: usize =
        8 + // discriminator
        32 + // owner
        32 + // yi_program
        32 + // yi_token
        32 + // yi_underlying_mint
        32 // yi_mint
        ;
}