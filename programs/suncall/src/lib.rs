mod instructions;
mod states;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7BmTamq2R1kVjrtv52Yivik5MXrKrseesMsbqK2heWqL");

#[program]
pub mod suncall {
    use super::*;

    #[access_control(Initialize::validate(&ctx))]
    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        instructions::initialize::handler(ctx, name)
    }

    #[access_control(UserLedgerReferenceInitialize::validate(&ctx))]
    pub fn user_ledger_reference_initialize(ctx: Context<UserLedgerReferenceInitialize>) -> Result<()> {
        instructions::user_ledger_reference_initialize::handler(ctx)
    }

    #[access_control(DepositInitialize::validate(&ctx))]
    pub fn deposit_initialize(ctx: Context<DepositInitialize>) -> Result<()> {
        instructions::deposit_initialize::handler(ctx)
    }

    #[access_control(Deposit::validate(&ctx))]
    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        instructions::deposit::handler(ctx)
    }

    #[access_control(Withdraw::validate(&ctx))]
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw::handler(ctx)
    }
}
