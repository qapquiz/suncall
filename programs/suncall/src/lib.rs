mod instructions;
mod states;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GaGdcADMhbCFCigsyMTRLUj2P876wCD1RqDCNvLrE7fh");

#[program]
pub mod suncall {
    use super::*;

    #[access_control(ctx.accounts.validate())]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }
}
