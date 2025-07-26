use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::add_validator_to_pool,
    std::num::NonZeroU32,
};

#[derive(Accounts)]
pub struct AddValidatorToPool<'info> {
    // [w] Stake pool
    // [s] Staker
    // [w] Reserve stake account
    // [] Stake pool withdraw authority
    // [w] Validator stake list storage account
    // [w] Stake account to add to the pool
    // [] Validator this stake account will be delegated to

    // userdata: optional non-zero u32 seed used for generating the validator stake address
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub staker: Signer<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub reserve: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub stake_pool_withdraw: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub validator: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_rent: Sysvar<'info, Rent>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_clock: Sysvar<'info, Clock>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_stake_history: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub sysvar_stake_config: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub system_program: Program<'info, System>,

    /// CHECKED: stake program id
    pub stake_program: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> AddValidatorToPool<'info> {
    // (Staker only) Adds stake account delegated to validator to the pool’s list of managed validators.
    pub fn context_handler(
        ctx: Context<AddValidatorToPool>,
        // userdata: optional non-zero u32 seed used for generating the validator stake address
        seed: Option<u32>,
    ) -> Result<()> {
        let ix = add_validator_to_pool(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.staker.key()),
            &get_pubkey(ctx.accounts.reserve.key()),
            &get_pubkey(ctx.accounts.stake_pool_withdraw.key()),
            &get_pubkey(ctx.accounts.validator_list.key()),
            &get_pubkey(ctx.accounts.stake.key()),
            &get_pubkey(ctx.accounts.validator.key()),
            seed.is_some()
                .then(|| NonZeroU32::new(seed.unwrap()).expect("expected NonZeroU32")),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // The stake account will have the rent-exempt amount plus max(
            //  crate::MINIMUM_ACTIVE_STAKE,
            //  solana_program::stake::tools::get_minimum_delegation()
            // ). It is funded from the stake pool reserve.

            // [w] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Staker
            ctx.accounts.staker.to_account_info(),
            // [w] Reserve stake account
            ctx.accounts.reserve.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw.to_account_info(),
            // [w] Validator stake list storage account
            ctx.accounts.validator_list.to_account_info(),
            // [w] Stake account to add to the pool
            ctx.accounts.stake.to_account_info(),
            // [] Validator this stake account will be delegated to
            ctx.accounts.validator.to_account_info(),
            // [] Rent sysvar
            ctx.accounts.sysvar_rent.to_account_info(),
            // [] Clock sysvar
            ctx.accounts.sysvar_clock.to_account_info(),
            // ‘[]’ Stake history sysvar
            ctx.accounts.sysvar_stake_history.to_account_info(),
            // ‘[]’ Stake config sysvar
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
