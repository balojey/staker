use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::update_stake_pool_balance,
};

#[derive(Accounts)]
pub struct UpdateStakePoolBalance<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub reserve_stake: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub manager_fee_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool_mint: UncheckedAccount<'info>,

    /// CHECKED: stake program id
    pub token_program: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> UpdateStakePoolBalance<'info> {
    // Updates total pool balance based on balances in the reserve and validator list
    pub fn context_handler(ctx: Context<UpdateStakePoolBalance>) -> Result<()> {
        let ix = update_stake_pool_balance(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.withdraw_authority.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
            &get_pubkey(ctx.accounts.reserve_stake.key()),
            &get_pubkey(ctx.accounts.manager_fee_account.key()),
            &get_pubkey(ctx.accounts.stake_pool_mint.key()),
            &get_pubkey(ctx.accounts.token_program.key()),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.withdraw_authority.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list.to_account_info(),
            // [] Reserve stake account
            ctx.accounts.reserve_stake.to_account_info(),
            // [w] Account to receive pool fee tokens
            ctx.accounts.manager_fee_account.to_account_info(),
            // [w] Pool mint account
            ctx.accounts.stake_pool_mint.to_account_info(),
            // [] Pool token program
            ctx.accounts.token_program.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
