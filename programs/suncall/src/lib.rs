mod instructions;
mod states;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7BmTamq2R1kVjrtv52Yivik5MXrKrseesMsbqK2heWqL");

#[program]
pub mod suncall {
    use super::*;

    #[access_control(Initialize::validate(&ctx))]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }
}
