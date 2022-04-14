use anchor_lang::prelude::*;

declare_id!("GaGdcADMhbCFCigsyMTRLUj2P876wCD1RqDCNvLrE7fh");

#[program]
pub mod suncall {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
