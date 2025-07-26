// DecreaseValidatorStake
// NOTE: This instruction has been deprecated since version 0.7.0.
// Please use DecreaseValidatorStakeWithReserve instead.

// (Staker only) Decrease active stake on a validator, eventually moving it to the reserve

// Internally, this instruction splits a validator stake account into its corresponding transient stake account
// and deactivates it.

// In order to rebalance the pool without taking custody,
// the staker needs a way of reducing the stake on a stake account.
// This instruction splits some amount of stake, up to the total activated stake,
// from the canonical validator stake account, into its “transient” stake account.

// The instruction only succeeds if the transient stake account does not exist.
// The amount of lamports to move must be at least rent-exemption plus
// max(crate::MINIMUM_ACTIVE_STAKE, solana_program::stake::tools::get_minimum_delegation()).

pub struct DecreaseValidatorStake<'info> {
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub staker: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_stake: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub transient_stake: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_clock: Sysvar<'info, Clock>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_rent: Sysvar<'info, Rent>,

    /// CHECKED: account required for the stake pool program
    pub system_program: Program<'info, System>,

    /// CHECKED: stake program id
    pub stake_program: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> DecreaseValidatorStake<'info> {
    pub fn context_handler(
        ctx: Context<DecreaseValidatorStake>,
        // amount of lamports to split into the transient stake account
        lamports: u64,
        // seed used to create transient stake account
        transient_stake_seed: u64,
    ) -> Result<()> {
        let ix = decrease_validator_stake(
            ctx.accounts.stake_pool_program.key(),
            ctx.accounts.stake_pool.key(),
            ctx.accounts.staker.key(),
            ctx.accounts.stake_pool_withdraw_authority.key(),
            ctx.accounts.validator_list.key(),
            ctx.accounts.validator_stake.key(),
            ctx.accounts.transient_stake.key(),
            lamports,
            transient_stake_seed,
        );

        let mut account_infos: Vec<AccountInfo> = vec![
            // [] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Stake pool staker
            ctx.accounts.staker.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Validator list
            ctx.accounts.validator_list.to_account_info(),
            // [w] Canonical stake account to split from
            ctx.accounts.validator_stake.to_account_info(),
            // [w] Transient stake account to receive split
            ctx.accounts.transient_stake_account.to_account_info(),
            // [] Clock sysvar
            ctx.accounts.sysvar_clock.to_account_info(),
            // [] Rent sysvar
            ctx.accounts.sysvar_rent.to_account_info(),
            // [] System program
            ctx.accounts.system_program.to_account_info(),
            // [] Stake program
            ctx.accounts.stake_program.to_account_info(),
        ];

        invoke(&ix, &account_infos);

        Ok(())
    }
}
