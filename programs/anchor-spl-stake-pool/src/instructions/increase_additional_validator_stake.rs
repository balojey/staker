// IncreaseAdditionalValidatorStake

// Works regardless if the transient stake account exists.

// Internally, this instruction splits reserve stake into an ephemeral stake account, activates it, then merges or splits it into the transient stake account delegated to the appropriate validator. UpdateValidatorListBalance will do the work of merging once it’s ready.

// The minimum amount to move is rent-exemption plus max(crate::MINIMUM_ACTIVE_STAKE, solana_program::stake::tools::get_minimum_delegation()).

// userdata: amount of lamports to increase on the given validator.

// The actual amount split into the transient stake account is: lamports + stake_rent_exemption.

// The rent-exemption of the stake account is withdrawn back to the reserve after it is merged.

use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::increase_additional_validator_stake,
};

#[derive(Accounts)]
pub struct IncreaseAdditionalValidatorStake<'info> {
    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub staker: Signer<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub reserve_stake: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub ephemeral_stake: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub transient_stake: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub validator_stake: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub validator: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_clock: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_stake_history: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_stake_config: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub system_program: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_program: UncheckedAccount<'info>,

    #[account(
        constraint = spl_stake_pool::check_id(&get_pubkey(stake_pool_program.key()))
    )]
    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> IncreaseAdditionalValidatorStake<'info> {
    // (Staker only) Increase stake on a validator again in an epoch.
    pub fn context_handler(
        ctx: Context<IncreaseAdditionalValidatorStake>,
        // amount of lamports to split into the transient stake account
        lamports: u64,
        // seed used to create transient stake account
        transient_stake_seed: u64,
        // seed used to create ephemeral account.
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        // userdata: amount of lamports to increase on the given validator.
        let ix = increase_additional_validator_stake(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.staker.key()),
            &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
            &get_pubkey(ctx.accounts.reserve_stake.key()),
            &get_pubkey(ctx.accounts.ephemeral_stake.key()),
            &get_pubkey(ctx.accounts.transient_stake.key()),
            &get_pubkey(ctx.accounts.validator_stake.key()),
            &get_pubkey(ctx.accounts.validator.key()),
            lamports,
            transient_stake_seed,
            ephemeral_stake_seed,
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Stake pool staker
            ctx.accounts.staker.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Validator list
            ctx.accounts.validator_list.to_account_info(),
            // [w] Stake pool reserve stake
            ctx.accounts.reserve_stake.to_account_info(),
            // [w] Uninitialized ephemeral stake account to receive stake
            ctx.accounts.ephemeral_stake.to_account_info(),
            // [w] Transient stake account
            ctx.accounts.transient_stake.to_account_info(),
            // [] Validator stake account
            ctx.accounts.validator_stake.to_account_info(),
            // [] Validator vote account to delegate to
            ctx.accounts.validator.to_account_info(),
            // ‘[]’ Clock sysvar
            ctx.accounts.sysvar_clock.to_account_info(),
            // [] Stake History sysvar
            ctx.accounts.sysvar_stake_history.to_account_info(),
            // [] Stake Config sysvar
            ctx.accounts.sysvar_stake_config.to_account_info(),
            // [] System program
            ctx.accounts.system_program.to_account_info(),
            // [] Stake program
            ctx.accounts.stake_program.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
