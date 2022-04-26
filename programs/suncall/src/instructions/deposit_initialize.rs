use anchor_lang::prelude::*;

use crate::states::{Lotto, UserLedger, UserLedgerReference};

#[derive(Accounts)]
pub struct DepositInitialize<'info> {
    pub lotto: Box<Account<'info, Lotto>>,
    #[account(
        seeds = [
            lotto.key().as_ref(),
            user.key().as_ref(),
            "ledger".as_bytes(),
        ],
        bump,
    )]
    pub user_ledger_reference: Box<Account<'info, UserLedgerReference>>,
    #[account(
        init,
        payer = user,
        space = UserLedger::LEN,
        seeds = [
            lotto.key().as_ref(),
            user_ledger_reference.array_index.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub user_ledger: Box<Account<'info, UserLedger>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositInitialize<'info> {
    pub fn validate(_ctx: &Context<DepositInitialize>) -> Result<()> {
        Ok(())
    }
}

pub fn handler(ctx: Context<DepositInitialize>) -> Result<()> {
    let user = &ctx.accounts.user;
    let user_ledger = &mut ctx.accounts.user_ledger;
    user_ledger.owner = user.key();
    user_ledger.total_yi_underlying_deposit = 0;
    user_ledger.total_yi_deposit = 0;
    Ok(())
}