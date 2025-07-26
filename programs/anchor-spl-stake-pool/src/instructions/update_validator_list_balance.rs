// While going through the pairs of validator and transient stake accounts,
// if the transient stake is inactive, it is merged into the reserve stake account.
// If the transient stake is active and has matching credits observed,
// it is merged into the canonical validator stake account. In all other states, nothing is done,
// and the balance is simply added to the canonical stake account balance.

use {
    crate::utils::{data::get_validator_list_data, get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::update_validator_list_balance_chunk,
    std::mem::transmute,
};

#[derive(Accounts)]
pub struct UpdateValidatorListBalance<'info> {
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub reserve_stake: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_clock: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_stake_history: UncheckedAccount<'info>,

    /// CHECKED: stake program id
    pub stake_program: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> UpdateValidatorListBalance<'info> {
    // Updates balances of validator and transient stake accounts in the pool
    pub fn context_handler(
        ctx: Context<UpdateValidatorListBalance>,
        // Index to start updating on the validator list
        start_index: u32,
        // If true, don’t try merging transient stake accounts into the reserve or validator stake account.
        // Useful for testing or if a particular stake account is in a bad state, but we still want to update
        no_merge: bool,
    ) -> Result<()> {
        let validator_list = &get_validator_list_data(&ctx.accounts.validator_list)?;
        let len = validator_list.validators.len();
        let ix = update_validator_list_balance_chunk(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
            &get_pubkey(ctx.accounts.reserve_stake.key()),
            validator_list,
            len,
            start_index as usize,
            no_merge,
        )
        // need to handle error
        .unwrap();

        let ix = get_instruction(ix);

        let mut account_infos: Vec<AccountInfo> = vec![
            // [] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list.to_account_info(),
            // [w] Reserve stake account
            ctx.accounts.reserve_stake.to_account_info(),
            // [] Sysvar clock
            ctx.accounts.sysvar_clock.to_account_info(),
            // [] Sysvar stake history
            ctx.accounts.sysvar_stake_history.to_account_info(),
            // [] Stake program
            ctx.accounts.stake_program.to_account_info(),
        ];

        let validation_list: Vec<AccountInfo> = unsafe {
            transmute(
                ctx.remaining_accounts
                    .iter()
                    .map(|account| account.clone())
                    .collect::<Vec<AccountInfo>>(),
            )
        };
        // ..7+2N ` [] N pairs of validator and transient stake accounts
        account_infos.extend(validation_list);

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
