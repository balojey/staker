use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::{set_preferred_validator, PreferredValidatorType},
};

#[derive(Accounts)]
pub struct SetPreferredValidator<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub staker: Signer<'info>,

    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> SetPreferredValidator<'info> {
    // (Staker only) Set the preferred deposit or withdraw stake account for the stake pool
    // In order to avoid users abusing the stake pool as a free conversion between SOL staked on different validators,
    // the staker can force all deposits and/or withdraws to go to one chosen account, or unset that account.
    pub fn context_handler(
        ctx: Context<SetPreferredValidator>,
        // Affected operation (deposit or withdraw)
        validator_type: PreferredValidatorType,
        // Validator vote account that deposits or withdraws must go through, unset with None
        validator_vote_address: Option<Pubkey>,
    ) -> Result<()> {
        let ix = set_preferred_validator(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.staker.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
            validator_type,
            validator_vote_address
                .is_some()
                .then(|| get_pubkey(validator_vote_address.unwrap())),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Stake pool staker
            ctx.accounts.staker.to_account_info(),
            // [] Validator list
            ctx.accounts.validator_list.to_account_info(),
            // Fails if the validator is not part of the stake pool.
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
