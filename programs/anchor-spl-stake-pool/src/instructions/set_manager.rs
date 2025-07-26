use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::set_manager,
};

#[derive(Accounts)]
pub struct SetManager<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    pub manager: Signer<'info>,

    pub new_manager: Signer<'info>,

    /// CHECKED: account required for the stake pool program
    pub new_fee_receiver: UncheckedAccount<'info>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> SetManager<'info> {
    // (Manager only) Update manager
    pub fn context_handler(ctx: Context<SetManager>) -> Result<()> {
        let ix = set_manager(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.manager.key()),
            &get_pubkey(ctx.accounts.new_manager.key()),
            &get_pubkey(ctx.accounts.new_fee_receiver.key()),
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [w] StakePool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Manager
            ctx.accounts.manager.to_account_info(),
            // [s] New manager
            ctx.accounts.new_manager.to_account_info(),
            // [] New manager fee account
            ctx.accounts.new_fee_receiver.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
