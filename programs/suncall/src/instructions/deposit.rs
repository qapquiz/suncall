use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};
use yi::YiToken;

use crate::states::{Lotto, UserLedgerReference, UserLedger, LottoState};

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub lotto: Box<Account<'info, Lotto>>,
    #[account(
        seeds = [
            lotto.key().as_ref(),
            "authority".as_bytes(),
        ],
        bump,
    )]
    /// CHECK: authority of the [lotto]
    pub lotto_authority: AccountInfo<'info>,
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
        seeds = [
            lotto.key().as_ref(),
            user.key().as_ref(),
            "ledger".as_bytes(),
        ],
        bump,  
    )]
    pub user_ledger_reference: Box<Account<'info, UserLedgerReference>>,
    #[account(
        mut,
        seeds = [
            lotto.key().as_ref(),
            user_ledger_reference.array_index.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub user_ledger: Box<Account<'info, UserLedger>>,
    #[account(
        address = lotto.yi_underlying_mint,
    )]
    pub yi_underlying_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        address = lotto.yi_mint,
    )]
    pub yi_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = yi_underlying_mint,
        associated_token::authority = user,
    )]
    pub user_yi_underlying_ata: Box<Account<'info, TokenAccount>>,
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
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn validate(_ctx: &Context<Deposit>, amount: u64) -> Result<()> {
        require_gt!(amount, 0);
        Ok(())
    }

    pub fn transfer_from_user_to_platform(&self, amount: u64, lotto_authority_bump: u8) -> Result<()> {
        let lotto_key = self.lotto.to_account_info().key();

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.user_yi_underlying_ata.to_account_info(),
            to: self.lotto_yi_underlying_ata.to_account_info(),
            authority: self.lotto_authority.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[lotto_key.as_ref(), "authority".as_bytes(), &[lotto_authority_bump]]];
        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds,
        );

        anchor_spl::token::transfer(cpi_context, amount)
    }

    pub fn yi_stake(&self, amount: u64, lotto_authority_bump: u8) -> Result<()> {
        let lotto_key = self.lotto.to_account_info().key();

        let cpi_program = self.yi_program.to_account_info();
        let cpi_accounts = yi::cpi::accounts::Stake {
            yi_token: self.yi_token.to_account_info(),
            yi_mint: self.yi_mint.to_account_info(),
            source_tokens: self.lotto_yi_underlying_ata.to_account_info(),
            source_authority: self.lotto_authority.to_account_info(),
            yi_underlying_tokens: self.yi_underlying_ata.to_account_info(),
            destination_yi_tokens: self.lotto_yi_ata.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[lotto_key.as_ref(), "authority".as_bytes(), &[lotto_authority_bump]]];

        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds
        );

        yi::cpi::stake(cpi_context, amount)
    }
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let lotto_authority_bump = *ctx.bumps.get("lotto_authority").unwrap();

    ctx.accounts.transfer_from_user_to_platform(amount, lotto_authority_bump)?;

    let lotto_yi_amount_before_deposit = ctx.accounts.lotto_yi_ata.amount;

    ctx.accounts.yi_stake(amount, lotto_authority_bump)?;

    // reload account to set the lamport after the stake
    ctx.accounts.lotto_yi_ata.reload()?;

    let lotto_yi_amount_after_deposit = ctx.accounts.lotto_yi_ata.amount;
    require_gt!(lotto_yi_amount_after_deposit, lotto_yi_amount_before_deposit);
    let lotto_yi_amount_diff = lotto_yi_amount_after_deposit.checked_sub(lotto_yi_amount_before_deposit).unwrap();

    let lotto_state = &mut ctx.accounts.lotto_state;
    lotto_state.total_yi_underlying_deposit = lotto_state.total_yi_underlying_deposit.checked_add(amount).unwrap();
    lotto_state.total_yi_deposit = lotto_yi_amount_after_deposit;

    let user_ledger = &mut ctx.accounts.user_ledger;
    user_ledger.total_yi_underlying_deposit = user_ledger.total_yi_underlying_deposit.checked_add(amount).unwrap();
    user_ledger.total_yi_deposit = user_ledger.total_yi_deposit.checked_add(lotto_yi_amount_diff).unwrap();
    Ok(())
}
