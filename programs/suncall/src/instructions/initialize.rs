use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};
use yi::YiToken;

use crate::states::{Lotto, LottoState};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = Lotto::LEN,
        seeds = [
            owner.key().as_ref(),
            name.as_bytes(),
            "lotto".as_bytes(),
        ],
        bump,
    )]
    pub lotto: Box<Account<'info, Lotto>>,
    #[account(
        seeds = [
            lotto.key().as_ref(),
            "authority".as_bytes(),
        ],
        bump
    )]
    /// CHECK: authority of the [lotto]
    pub lotto_authority: AccountInfo<'info>,
    #[account(
        init,
        payer = owner,
        space = LottoState::LEN,
        seeds = [
            lotto.key().as_ref(),
            "state".as_bytes(),
        ],
        bump,
    )]
    pub lotto_state: Box<Account<'info, LottoState>>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = yi_underlying_mint,
        associated_token::authority = lotto_authority,
    )]
    pub lotto_yi_underlying_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = yi_mint,
        associated_token::authority = lotto_authority,
    )]
    pub lotto_yi_ata: Box<Account<'info, TokenAccount>>,
    pub yi_token: AccountLoader<'info, YiToken>,
    pub yi_underlying_mint: Box<Account<'info, Mint>>,
    pub yi_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn validate(_ctx: &Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

pub fn handler(ctx: Context<Initialize>, _name: String) -> Result<()> {
    let owner_pubkey = ctx.accounts.owner.key();
    let yi_token_pubkey = ctx.accounts.yi_token.key();
    let yi_underlying_mint_pubkey = ctx.accounts.yi_underlying_mint.key();
    let yi_mint_pubkey = ctx.accounts.yi_mint.key();

    let lotto = &mut ctx.accounts.lotto;
    lotto.owner = owner_pubkey;
    lotto.yi_program = yi::id();
    lotto.yi_token = yi_token_pubkey;
    lotto.yi_underlying_mint = yi_underlying_mint_pubkey;
    lotto.yi_mint = yi_mint_pubkey;

    let lotto_state = &mut ctx.accounts.lotto_state;
    lotto_state.total_yi_underlying_deposit = 0;
    lotto_state.total_yi_deposit = 0;
    lotto_state.round = 1;
    lotto_state.user_ledgers_len = 0;

    Ok(())
}