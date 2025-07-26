use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::{
        withdraw_sol_with_authority_and_slippage, withdraw_sol_with_slippage,
    },
};

#[derive(Accounts)]
pub struct WithdrawSolWithSlippage<'info> {
    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub user_transfer_authority: Signer<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub pool_tokens_from: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub reserve_stake_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub lamports_to: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub manager_fee_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub pool_mint: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_clock: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_stake_history: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_program: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub token_program: UncheckedAccount<'info>,

    /// CHECKED: account is optional and validated on the stake pool program
    pub sol_withdraw_authority: Option<Signer<'info>>,

    /// CHECKED: account is optional and validated on the stake pool program
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> WithdrawSolWithSlippage<'info> {
    // Withdraw SOL directly from the pool’s reserve account. Fails if the reserve does not have enough SOL or if the slippage constraint is not met.
    pub fn context_handler(
        ctx: Context<WithdrawSolWithSlippage>,
        // Pool tokens to burn in exchange for lamports
        pool_tokens_in: u64,

        // Minimum amount of lamports that must be received
        minimum_lamports_out: u64,
    ) -> Result<()> {
        let ix = if let Some(sol_withdraw_authority) = ctx.accounts.sol_withdraw_authority.clone() {
            let ix = withdraw_sol_with_authority_and_slippage(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(sol_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.user_transfer_authority.key()),
                &get_pubkey(ctx.accounts.pool_tokens_from.key()),
                &get_pubkey(ctx.accounts.reserve_stake_account.key()),
                &get_pubkey(ctx.accounts.lamports_to.key()),
                &get_pubkey(ctx.accounts.manager_fee_account.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                pool_tokens_in,
                minimum_lamports_out,
            );

            ix
        } else {
            let ix = withdraw_sol_with_slippage(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.user_transfer_authority.key()),
                &get_pubkey(ctx.accounts.pool_tokens_from.key()),
                &get_pubkey(ctx.accounts.reserve_stake_account.key()),
                &get_pubkey(ctx.accounts.lamports_to.key()),
                &get_pubkey(ctx.accounts.manager_fee_account.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                pool_tokens_in,
                minimum_lamports_out,
            );

            ix
        };

        let ix = get_instruction(ix);

        let mut account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [s] User transfer authority, for pool token account
            ctx.accounts.user_transfer_authority.to_account_info(),
            // [w] User account to burn pool tokens
            ctx.accounts.pool_tokens_from.to_account_info(),
            // [w] Reserve stake account, to withdraw SOL
            ctx.accounts.reserve_stake_account.to_account_info(),
            // [w] Account receiving the lamports from the reserve, must be a system account
            ctx.accounts.lamports_to.to_account_info(),
            // [w] Account to receive pool fee tokens
            ctx.accounts.manager_fee_account.to_account_info(),
            // [w] Pool token mint account
            ctx.accounts.pool_mint.to_account_info(),
            // ‘[]’ Clock sysvar
            ctx.accounts.sysvar_clock.to_account_info(),
            // ‘[]’ Stake history sysvar
            ctx.accounts.sysvar_stake_history.to_account_info(),
            // [] Stake program account
            ctx.accounts.stake_program.to_account_info(),
            // [] Token program id
            ctx.accounts.token_program.to_account_info(),
        ];

        if ctx.accounts.sol_withdraw_authority.is_some() {
            // [s] (Optional) Stake pool sol deposit authority.
            account_infos.push(
                ctx.accounts
                    .sol_withdraw_authority
                    .clone()
                    .unwrap()
                    .to_account_info(),
            );
        }

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
