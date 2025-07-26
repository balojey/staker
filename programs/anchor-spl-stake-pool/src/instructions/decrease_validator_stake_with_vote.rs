// Internally, this instruction:

// 1. withdraws enough lamports to make the transient account rent-exempt
// 2. splits from a validator stake account into a transient stake account
// 3. deactivates the transient stake account
// 4. In order to rebalance the pool without taking custody, the staker needs a way of reducing the stake on a stake account.
// This instruction splits some amount of stake, up to the total activated stake, from the canonical validator stake account, into its “transient” stake account.

// The instruction only succeeds if the transient stake account does not exist.
// The amount of lamports to move must be at least rent-exemption
// plus max(crate::MINIMUM_ACTIVE_STAKE, solana_program::stake::tools::get_minimum_delegation()).

use {
    crate::utils::{data::get_stake_pool_data, get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::decrease_validator_stake_with_vote,
    std::num::NonZeroU32,
};

#[derive(Accounts)]
pub struct DecreaseValidatorStakeWithVote<'info> {
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
    pub validator_stake: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required and validated on the stake pool program
    pub transient_stake: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_clock: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub sysvar_stake_history: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub system_program: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_program: UncheckedAccount<'info>,

    // need validaite program ID
    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> DecreaseValidatorStakeWithVote<'info> {
    // (Staker only) Decrease active stake on a validator, eventually moving it to the reserve
    pub fn context_handler(
        ctx: Context<DecreaseValidatorStakeWithVote>,
        // vote account
        vote_account_address: Pubkey,
        // amount of lamports to split into the transient stake account
        lamports: u64,
        // validator stake seed
        validator_stake_seed: Option<u32>,
        // seed used to create transient stake account
        transient_stake_seed: u64,
    ) -> Result<()> {
        let ix = decrease_validator_stake_with_vote(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_stake_pool_data(&ctx.accounts.stake_pool)?,
            &get_pubkey(ctx.accounts.staker.key()),
            &get_pubkey(vote_account_address),
            lamports,
            validator_stake_seed.is_some().then(|| {
                NonZeroU32::new(validator_stake_seed.unwrap()).expect("expected NonZeroU32")
            }),
            transient_stake_seed,
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
            // [w] Reserve stake account, to fund rent exempt reserve
            ctx.accounts.reserve_stake.to_account_info(),
            // [w] Canonical stake account to split from
            ctx.accounts.validator_stake.to_account_info(),
            // [w] Transient stake account to receive split, must not exist?
            ctx.accounts.transient_stake.to_account_info(),
            // [] Clock sysvar
            ctx.accounts.sysvar_clock.to_account_info(),
            // ‘[]’ Stake history sysvar
            ctx.accounts.sysvar_stake_history.to_account_info(),
            // [] System program
            ctx.accounts.system_program.to_account_info(),
            // [] Stake program
            ctx.accounts.stake_program.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
