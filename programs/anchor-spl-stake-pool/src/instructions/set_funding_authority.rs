use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::{set_funding_authority, FundingType},
};

#[derive(Accounts)]
pub struct SetFundingAuthority<'info> {
    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    pub manager: Signer<'info>,

    /// CHECKED: account required for the stake pool program
    pub new_sol_deposit_authority: Option<UncheckedAccount<'info>>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> SetFundingAuthority<'info> {
    // (Manager only) Update SOL deposit, stake deposit, or SOL withdrawal authority.
    pub fn context_handler(
        ctx: Context<SetFundingAuthority>,
        funding_type: FundingType,
    ) -> Result<()> {
        let ix = if let Some(new_sol_deposit_authority) =
            ctx.accounts.new_sol_deposit_authority.clone()
        {
            let new_sol_deposit_authority = get_pubkey(new_sol_deposit_authority.key());
            let ix = set_funding_authority(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(ctx.accounts.manager.key()),
                Some(&new_sol_deposit_authority),
                funding_type,
            );
            ix
        } else {
            let ix = set_funding_authority(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(ctx.accounts.manager.key()),
                None,
                funding_type,
            );

            ix
        };

        let ix = get_instruction(ix);

        let mut account_infos: Vec<AccountInfo> = vec![
            // [w] StakePool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Manager
            ctx.accounts.manager.to_account_info(),
        ];

        // ’[]` New authority pubkey or none
        if ctx.accounts.new_sol_deposit_authority.is_some() {
            account_infos.push(
                ctx.accounts
                    .new_sol_deposit_authority
                    .as_ref()
                    .unwrap()
                    .to_account_info(),
            )
        }

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
