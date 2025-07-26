use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::{instruction::set_fee, state::FeeType},
};

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    pub manager: Signer<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> SetFee<'info> {
    // (Manager only) Update manager
    pub fn context_handler(
        ctx: Context<SetFee>,
        // Type of fee to update and value to update it to
        fee: FeeType,
    ) -> Result<()> {
        let ix = set_fee(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.manager.key()),
            fee,
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] StakePool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Manager
            ctx.accounts.manager.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
