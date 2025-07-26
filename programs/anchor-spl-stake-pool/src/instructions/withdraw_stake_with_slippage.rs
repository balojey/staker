use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::withdraw_stake_with_slippage,
};

#[derive(Accounts)]
pub struct WithdrawStakeWithSlippage<'info> {
    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub validator_list_storage: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool_withdraw: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub stake_to_split: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub stake_to_receive: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub user_stake_authority: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub user_transfer_authority: Signer<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub user_pool_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub manager_fee_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub pool_mint: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_clock: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub token_program: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_program: UncheckedAccount<'info>,

    /// CHECKED: account is optional and validated on the stake pool program
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> WithdrawStakeWithSlippage<'info> {
    // Withdraw the token from the pool at the current ratio, specifying a minimum expected output lamport amount.
    // Succeeds if the stake account has enough SOL to cover the desired amount of pool tokens, and if the withdrawal keeps the total staked amount above the minimum of rent-exempt amount + max(    crate::MINIMUM_ACTIVE_STAKE,    solana_program::stake::tools::get_minimum_delegation()  ).
    pub fn context_handler(
        ctx: Context<WithdrawStakeWithSlippage>,
        // Pool tokens to burn in exchange for lamports
        pool_tokens_in: u64,
        // Minimum amount of lamports that must be received
        minimum_lamports_out: u64,
    ) -> Result<()> {
        let ix = withdraw_stake_with_slippage(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.validator_list_storage.key()),
            &get_pubkey(ctx.accounts.stake_pool_withdraw.key()),
            &get_pubkey(ctx.accounts.stake_to_split.key()),
            &get_pubkey(ctx.accounts.stake_to_receive.key()),
            &get_pubkey(ctx.accounts.user_stake_authority.key()),
            &get_pubkey(ctx.accounts.user_transfer_authority.key()),
            &get_pubkey(ctx.accounts.user_pool_token_account.key()),
            &get_pubkey(ctx.accounts.manager_fee_account.key()),
            &get_pubkey(ctx.accounts.pool_mint.key()),
            &get_pubkey(ctx.accounts.token_program.key()),
            pool_tokens_in,
            minimum_lamports_out,
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list_storage.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw.to_account_info(),
            // [w] Validator or reserve stake account to split
            ctx.accounts.stake_to_split.to_account_info(),
            // [w] Uninitialized stake account to receive withdrawal
            ctx.accounts.stake_to_receive.to_account_info(),
            // [] User account to set as a new withdraw authority
            ctx.accounts.user_stake_authority.to_account_info(),
            // [s] User transfer authority, for pool token account
            ctx.accounts.user_transfer_authority.to_account_info(),
            // [w] User account with pool tokens to burn from
            ctx.accounts.user_pool_token_account.to_account_info(),
            // [w] Account to receive pool fee tokens
            ctx.accounts.manager_fee_account.to_account_info(),
            // [w] Pool token mint account
            ctx.accounts.pool_mint.to_account_info(),
            // [] Sysvar clock account (required)
            ctx.accounts.sysvar_clock.to_account_info(),
            // [] Pool token program id
            ctx.accounts.token_program.to_account_info(),
            // [] Stake program id,
            ctx.accounts.stake_program.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
