use anchor_lang::prelude::*;

use crate::states::Lotto;

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub lotto: Box<Account<'info, Lotto>>,
}

impl<'info> Deposit<'info> {
    pub fn validate(_ctx: &Context<Deposit>) -> Result<()> {
        Ok(())
    }
}

pub fn handler(ctx: Context<Deposit>) -> Result<()> {
    Ok(())
}