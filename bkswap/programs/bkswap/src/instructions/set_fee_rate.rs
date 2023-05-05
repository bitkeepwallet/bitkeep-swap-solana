use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct SetFeeRate<'info> {
    pub bkswap_account: Account<'info, Bkswap>,

    #[account(address = bkswap_account.authority)]
    pub authority: Signer<'info>,
}

pub fn set_fee_rate(ctx: Context<SetFeeRate>, fee_rate: u16) -> Result<()> {
    let bkswap_account = &mut ctx.accounts.bkswap_account;

    bkswap_account.fee_rate = fee_rate;
    Ok(())
}
