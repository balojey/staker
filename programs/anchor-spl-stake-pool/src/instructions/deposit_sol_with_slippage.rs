// DepositSolWithSlippage
// Deposit SOL directly into the pool’s reserve account, with a specified slippage constraint. The output is a “pool” token representing ownership into the pool. Inputs are converted at the current ratio.

// [w] Stake pool
// [] Stake pool withdraw authority
// [w] Reserve stake account, to deposit SOL
// [s] Account providing the lamports to be deposited into the pool
// [w] User account to receive pool tokens
// [w] Account to receive fee tokens
// [w] Account to receive a portion of fee as referral fees
// [w] Pool token mint account
// [] System program account
// [] Token program id
// [s] (Optional) Stake pool sol deposit authority.

use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::{
        deposit_sol_with_authority_and_slippage, deposit_sol_with_slippage,
    },
};

#[derive(Accounts)]
pub struct DepositSolWithSlippage<'info> {
    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub reserve_stake_account: UncheckedAccount<'info>,

    // [s] Account providing the lamports to be deposited into the pool
    /// CHECKED: account required and validated on the stake pool program
    pub lamports_from: Signer<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub pool_tokens_to: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub manager_fee_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub referrer_pool_tokens_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub pool_mint: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub system_program: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub token_program: UncheckedAccount<'info>,

    /// CHECKED: account is optional and validated on the stake pool program
    pub sol_deposit_authority: Option<Signer<'info>>,

    /// CHECKED: account is optional and validated on the stake pool program
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> DepositSolWithSlippage<'info> {
    // Deposit SOL directly into the pool’s reserve account. The output is a “pool” token representing ownership into the pool. Inputs are converted to the current ratio.
    pub fn context_handler(
        ctx: Context<DepositSolWithSlippage>,
        // Amount of lamports to deposit into the reserve
        lamports_in: u64,
        // Minimum amount of pool tokens that must be received
        minimum_pool_tokens_out: u64,
    ) -> Result<()> {
        let ix = if let Some(sol_deposit_authority) = ctx.accounts.sol_deposit_authority.clone() {
            let ix = deposit_sol_with_authority_and_slippage(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(sol_deposit_authority.key()),
                &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.reserve_stake_account.key()),
                &get_pubkey(ctx.accounts.lamports_from.key()),
                &get_pubkey(ctx.accounts.pool_tokens_to.key()),
                &get_pubkey(ctx.accounts.manager_fee_account.key()),
                &get_pubkey(ctx.accounts.referrer_pool_tokens_account.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                lamports_in,
                minimum_pool_tokens_out,
            );
            ix
        } else {
            let ix = deposit_sol_with_slippage(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.reserve_stake_account.key()),
                &get_pubkey(ctx.accounts.lamports_from.key()),
                &get_pubkey(ctx.accounts.pool_tokens_to.key()),
                &get_pubkey(ctx.accounts.manager_fee_account.key()),
                &get_pubkey(ctx.accounts.referrer_pool_tokens_account.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                lamports_in,
                minimum_pool_tokens_out,
            );
            ix
        };

        let ix = get_instruction(ix);

        let mut account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Reserve stake account, to deposit SOL
            ctx.accounts.reserve_stake_account.to_account_info(),
            // [s] Account providing the lamports to be deposited into the pool
            ctx.accounts.lamports_from.to_account_info(),
            // [w] User account to receive pool tokens
            ctx.accounts.pool_tokens_to.to_account_info(),
            // [w] Account to receive fee tokens
            ctx.accounts.manager_fee_account.to_account_info(),
            // [w] Account to receive a portion of fee as referral fees
            ctx.accounts.referrer_pool_tokens_account.to_account_info(),
            // [w] Pool token mint account
            ctx.accounts.pool_mint.to_account_info(),
            // [] System program account
            ctx.accounts.system_program.to_account_info(),
            // [] Token program id
            ctx.accounts.token_program.to_account_info(),
        ];

        if ctx.accounts.sol_deposit_authority.is_some() {
            // [s] (Optional) Stake pool sol deposit authority.
            account_infos.push(
                ctx.accounts
                    .sol_deposit_authority
                    .clone()
                    .unwrap()
                    .to_account_info(),
            );
        }

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
