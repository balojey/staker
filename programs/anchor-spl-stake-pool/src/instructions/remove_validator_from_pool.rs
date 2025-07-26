use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::remove_validator_from_pool,
};

#[derive(Accounts)]
pub struct RemoveValidatorFromPool<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub staker: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub transient_stake_account: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_clock: Sysvar<'info, Clock>,

    /// CHECKED: stake program id
    pub stake_program: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> RemoveValidatorFromPool<'info> {
    // (Staker only) Removes validator from the pool, deactivating its stake
    pub fn context_handler(ctx: Context<RemoveValidatorFromPool>) -> Result<()> {
        let ix = remove_validator_from_pool(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.staker.key()),
            &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
            &get_pubkey(ctx.accounts.stake_account.key()),
            &get_pubkey(ctx.accounts.transient_stake_account.key()),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Staker
            ctx.accounts.staker.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list.to_account_info(),
            // [w] Stake account to remove from the pool
            ctx.accounts.stake_account.to_account_info(),
            // [w] Transient stake account, to deactivate if necessary
            ctx.accounts.transient_stake_account.to_account_info(),
            // [] Sysvar clock
            ctx.accounts.sysvar_clock.to_account_info(),
            // [] Stake program id,
            ctx.accounts.stake_program.to_account_info(),
        ];

        // Only succeeds if the validator stake account has the minimum of
        // max(crate::MINIMUM_ACTIVE_STAKE, solana_program::stake::tools::get_minimum_delegation()).
        // plus the rent-exempt amount.

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
