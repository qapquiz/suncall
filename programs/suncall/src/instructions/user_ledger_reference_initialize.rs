use anchor_lang::prelude::*;

use crate::states::{Lotto, LottoState, UserLedgerReference};

#[derive(Accounts)]
pub struct UserLedgerReferenceInitialize<'info> {
    pub lotto: Box<Account<'info, Lotto>>,
    #[account(
        mut,
        seeds = [
            lotto.key().as_ref(),
            "state".as_bytes(),
        ],
        bump,
    )]
    pub lotto_state: Box<Account<'info, LottoState>>,
    #[account(
        init,
        payer = user,
        space = UserLedgerReference::LEN,
        seeds = [
            lotto.key().as_ref(),
            user.key().as_ref(),
            "ledger".as_bytes(),
        ],
        bump,
    )]
    pub user_ledger_reference: Box<Account<'info, UserLedgerReference>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UserLedgerReferenceInitialize<'info> {
    pub fn validate(_ctx: &Context<UserLedgerReferenceInitialize>) -> Result<()> {
        Ok(())
    }
}

pub fn handler(ctx: Context<UserLedgerReferenceInitialize>) -> Result<()> {
    let lotto_state = &mut ctx.accounts.lotto_state;
    let user_ledger = &mut ctx.accounts.user_ledger_reference;
    
    user_ledger.lotto = ctx.accounts.lotto.key();
    user_ledger.array_index = lotto_state.user_ledgers_len;

    lotto_state.user_ledgers_len = lotto_state.user_ledgers_len.checked_add(1).unwrap();
    
    Ok(())
}