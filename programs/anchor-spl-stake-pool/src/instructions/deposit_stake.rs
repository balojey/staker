use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::{deposit_stake, deposit_stake_with_authority},
};

#[derive(Accounts)]
pub struct DepositStake<'info> {
    // an authority required by the stake program
    pub deposit_stake_withdraw_authority: Signer<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list_storage: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program, optinal signer
    pub stake_pool_deposit_authority: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub deposit_stake_address: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_stake_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub reserve_stake_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub pool_tokens_to: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub manager_fee_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub referrer_pool_tokens_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub pool_mint: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_clock: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_stake_history: UncheckedAccount<'info>,

    /// CHECKED: token program id
    pub token_program: UncheckedAccount<'info>,

    /// CHECKED: stake program id
    pub stake_program: UncheckedAccount<'info>,

    // #[account(
    //     constraint = spl_stake_pool::check_id(&get_pubkey(stake_pool_program.key()))
    // )]
    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> DepositStake<'info> {
    // Deposit some stake into the pool. The output is a “pool” token representing ownership into the pool. Inputs are converted to the current ratio.
    pub fn context_handler(ctx: Context<DepositStake>) -> Result<()> {
        // expect to be 3 instructions to be invoked

        let ix_list = if ctx
            .accounts
            .stake_pool_deposit_authority
            .signer_key()
            .is_some()
        {
            // instruction version where stake pool deposit authority is signer
            deposit_stake_with_authority(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.validator_list_storage.key()),
                &get_pubkey(ctx.accounts.stake_pool_deposit_authority.key()),
                &get_pubkey(ctx.accounts.deposit_stake_address.key()),
                &get_pubkey(ctx.accounts.deposit_stake_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.validator_stake_account.key()),
                &get_pubkey(ctx.accounts.pool_tokens_to.key()),
                &get_pubkey(ctx.accounts.manager_fee_account.key()),
                &get_pubkey(ctx.accounts.referrer_pool_tokens_account.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
            )
        } else {
            // instruction version where stake pool deposit authority is not signer
            deposit_stake(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.validator_list_storage.key()),
                &get_pubkey(ctx.accounts.stake_pool_deposit_authority.key()),
                &get_pubkey(ctx.accounts.deposit_stake_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.validator_stake_account.key()),
                &get_pubkey(ctx.accounts.pool_tokens_to.key()),
                &get_pubkey(ctx.accounts.manager_fee_account.key()),
                &get_pubkey(ctx.accounts.referrer_pool_tokens_account.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
            )
        };

        let ix = get_instruction(ix_list[0].clone());
        let account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.deposit_stake_address.to_account_info(),
            ctx.accounts.sysvar_clock.to_account_info(),
            ctx.accounts
                .deposit_stake_withdraw_authority
                .to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        let ix = get_instruction(ix_list[1].clone());
        let account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.deposit_stake_address.to_account_info(),
            ctx.accounts.sysvar_clock.to_account_info(),
            ctx.accounts
                .deposit_stake_withdraw_authority
                .to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        let ix = get_instruction(ix_list[2].clone());
        let account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list_storage.to_account_info(),
            // [s]/[] Stake pool deposit authority
            ctx.accounts.stake_pool_deposit_authority.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Stake account to join the pool (withdraw authority for the stake account should be first set to the stake pool deposit authority)
            ctx.accounts.deposit_stake_address.to_account_info(),
            // [w] Validator stake account for the stake account to be merged with
            ctx.accounts.validator_stake_account.to_account_info(),
            // [w] Reserve stake account, to withdraw rent exempt reserve
            ctx.accounts.reserve_stake_account.to_account_info(),
            // [w] User account to receive pool tokens
            ctx.accounts.pool_tokens_to.to_account_info(),
            // [w] Account to receive pool fee tokens
            ctx.accounts.manager_fee_account.to_account_info(),
            // [w] Account to receive a portion of pool fee tokens as referral fees
            ctx.accounts.referrer_pool_tokens_account.to_account_info(),
            // [w] Pool token mint account
            ctx.accounts.pool_mint.to_account_info(),
            // ‘[]’ Sysvar clock account
            ctx.accounts.sysvar_clock.to_account_info(),
            // ‘[]’ Sysvar stake history account
            ctx.accounts.sysvar_stake_history.to_account_info(),
            // [] Pool token program id,
            ctx.accounts.token_program.to_account_info(),
            // [] Stake program id,
            ctx.accounts.stake_program.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
