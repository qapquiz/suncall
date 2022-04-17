use anchor_lang::prelude::*;

#[account]
pub struct UserLedgerReference {
    pub lotto: Pubkey,
    pub array_index: u64,
}

impl UserLedgerReference {
    pub const LEN: usize =
        8 + // discriminator
        32 + // lotto
        8 // array_index
        ;        
}