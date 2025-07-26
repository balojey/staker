// DecreaseAdditionalValidatorStakeWithList

// Works regardless if the transient stake account already exists.

// Internally, this instruction:

// withdraws rent-exempt reserve lamports from the reserve into the ephemeral stake
// splits a validator stake account into an ephemeral stake account
// deactivates the ephemeral account
// merges or splits the ephemeral account into the transient stake account delegated to the appropriate validator
// The amount of lamports to move must be at least
// max(crate::MINIMUM_ACTIVE_STAKE,
// solana_program::stake::tools::get_minimum_delegation()).

use {
    crate::utils::{
        data::{get_stake_pool_data, get_validator_list_data},
        get_instruction, get_pubkey,
    },
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::decrease_additional_validator_stake_with_list,
};

#[derive(Accounts)]
pub struct DecreaseAdditionalValidatorStakeWithList<'info> {
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
    pub ephemeral_stake: UncheckedAccount<'info>,

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

impl<'info> DecreaseAdditionalValidatorStakeWithList<'info> {
    // (Staker only) Decrease active stake again from a validator, eventually moving it to the reserve
    pub fn context_handler(
        ctx: Context<DecreaseAdditionalValidatorStakeWithList>,
        // vote account address
        vote_account: Pubkey,
        // amount of lamports to split into the transient stake account
        lamports: u64,
        // seed used to create ephemeral account.
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        let ix = decrease_additional_validator_stake_with_list(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_stake_pool_data(&ctx.accounts.stake_pool)?,
            &get_validator_list_data(&ctx.accounts.validator_list)?,
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(vote_account),
            lamports,
            ephemeral_stake_seed,
        )
        .unwrap();

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
            // [w] Uninitialized ephemeral stake account to receive stake
            ctx.accounts.ephemeral_stake.to_account_info(),
            // [w] Transient stake account
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
