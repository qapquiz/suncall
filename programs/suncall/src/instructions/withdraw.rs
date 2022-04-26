use anchor_lang::prelude::*;

use crate::states::Lotto;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub lotto: Box<Account<'info, Lotto>>,
}

impl<'info> Withdraw<'info> {
    pub fn validate(_ctx: &Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

pub fn handler(_ctx: Context<Withdraw>) -> Result<()> {
    Ok(())
}