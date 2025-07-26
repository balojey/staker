use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::set_staker,
};

#[derive(Accounts)]
pub struct SetStaker<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    pub set_staker_authority: Signer<'info>,

    /// CHECKED: account required for the stake pool program
    pub new_staker: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> SetStaker<'info> {
    // (Manager or staker only) Update staker
    pub fn context_handler(ctx: Context<SetStaker>) -> Result<()> {
        let ix = set_staker(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.set_staker_authority.key()),
            &get_pubkey(ctx.accounts.new_staker.key()),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] StakePool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Manager or current staker
            ctx.accounts.set_staker_authority.to_account_info(),
            // ’[]` New staker pubkey
            ctx.accounts.new_staker.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
