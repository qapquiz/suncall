use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};
use yi::YiToken;

use crate::states::Lotto;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub lotto: Box<Account<'info, Lotto>>,
    #[account(
        seeds = [
            lotto.key().as_ref(),
            "authority".as_bytes(),
        ],
        bump,
    )]
    /// CHECK: authority of the [Lotto],
    pub lotto_authority: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = yi_underlying_mint,
        associated_token::authority = lotto_authority,
    )]
    pub lotto_yi_underlying_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = yi_mint,
        associated_token::authority = lotto_authority,
    )]
    pub lotto_yi_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        address = lotto.yi_underlying_mint,
    )]
    pub yi_underlying_mint: Box<Account<'info, Mint>>,
    #[account(
        address = lotto.yi_mint,
    )]
    pub yi_mint: Box<Account<'info, Mint>>,
    #[account(
        address = lotto.yi_program,
    )]
    /// CHECK: program of the [Yi]
    pub yi_program: AccountInfo<'info>,
    #[account(
        address = lotto.yi_token,
    )]
    pub yi_token: AccountLoader<'info, YiToken>,
    #[account(
        associated_token::mint = yi_underlying_mint,
        associated_token::authority = yi_token,
    )]
    pub yi_underlying_ata: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn validate(_ctx: &Context<Withdraw>) -> Result<()> {
        Ok(())
    }

    pub fn yi_unstake(&self, yi_amount: u64) -> Result<()> {
        let cpi_program = self.yi_program.to_account_info();
        let cpi_accounts = yi::cpi::accounts::Unstake {
            yi_token: self.yi_token.to_account_info(),
            yi_mint: self.yi_mint.to_account_info(),
            source_yi_tokens: self.lotto_yi_ata.to_account_info(),
            source_authority: self.lotto_authority.to_account_info(),
            yi_underlying_tokens: self.yi_underlying_ata.to_account_info(),
            destination_underlying_tokens: self.lotto_yi_underlying_ata.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[]];
        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds
        );

        yi::cpi::unstake(cpi_context, yi_amount)
    }
}

pub fn handler(ctx: Context<Withdraw>, underlying_amount: u64) -> Result<()> {
    let yi_token = &ctx.accounts.yi_token.load()?;
    let yi_underlying_ata = &ctx.accounts.yi_underlying_ata;
    let yi_mint = &ctx.accounts.yi_mint;
    let yi_amount = yi_token.calculate_yitokens_for_underlying(
        underlying_amount,
        yi_underlying_ata.amount,
        yi_mint.supply,
    ).unwrap();
    ctx.accounts.yi_unstake(yi_amount)?;
    Ok(())
}