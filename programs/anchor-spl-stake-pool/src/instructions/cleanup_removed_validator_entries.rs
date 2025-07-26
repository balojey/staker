use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::cleanup_removed_validator_entries,
};

#[derive(Accounts)]
pub struct CleanupRemovedValidatorEntries<'info> {
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> CleanupRemovedValidatorEntries<'info> {
    // Cleans up validator stake account entries marked as ReadyForRemoval
    pub fn context_handler(ctx: Context<CleanupRemovedValidatorEntries>) -> Result<()> {
        let ix = cleanup_removed_validator_entries(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
