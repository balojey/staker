use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::{instruction::initialize, state::Fee},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    /// CHECKED: signer required for the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    /// CHECKED: signer required for the stake pool program
    pub manager: Signer<'info>,

    /// CHECKED: account required for the stake pool program
    pub staker: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECKED: account required for the stake pool program
    pub validator_list: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub reserve_stake: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub pool_mint: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub manager_pool_account: UncheckedAccount<'info>,

    // [] Token program id | Token 2022 id ?
    /// CHECKED: account required for the stake pool program
    pub token_program: UncheckedAccount<'info>,

    /// CHECKED: account required for the stake pool program
    pub deposit_authority: Option<UncheckedAccount<'info>>,

    /// CHECKED: stake pool program id
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> Initialize<'info> {
    // Initializes a new StakePool.
    pub fn context_handler(
        ctx: Context<Initialize>,
        // Fee assessed as percentage of perceived rewards
        fee: Fee,
        // Fee charged per withdrawal as percentage of withdrawal
        withdrawal_fee: Fee,
        // Fee charged per deposit as percentage of deposit
        deposit_fee: Fee,
        // Percentage [0-100] of deposit_fee that goes to referrer
        referral_fee: u8,
        // Maximum expected number of validators
        max_validators: u32,
    ) -> Result<()> {
        let ix = if let Some(deposit_authority) = ctx.accounts.deposit_authority.clone() {
            let deposit_authority = get_pubkey(deposit_authority.key());
            let ix = initialize(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(ctx.accounts.manager.key()),
                &get_pubkey(ctx.accounts.staker.key()),
                &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.validator_list.key()),
                &get_pubkey(ctx.accounts.reserve_stake.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.manager_pool_account.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                Some(deposit_authority),
                fee,
                withdrawal_fee,
                deposit_fee,
                referral_fee,
                max_validators,
            );
            ix
        } else {
            let ix = initialize(
                &get_pubkey(ctx.accounts.stake_pool_program.key()),
                &get_pubkey(ctx.accounts.stake_pool.key()),
                &get_pubkey(ctx.accounts.manager.key()),
                &get_pubkey(ctx.accounts.staker.key()),
                &get_pubkey(ctx.accounts.stake_pool_withdraw_authority.key()),
                &get_pubkey(ctx.accounts.validator_list.key()),
                &get_pubkey(ctx.accounts.reserve_stake.key()),
                &get_pubkey(ctx.accounts.pool_mint.key()),
                &get_pubkey(ctx.accounts.manager_pool_account.key()),
                &get_pubkey(ctx.accounts.token_program.key()),
                None,
                fee,
                withdrawal_fee,
                deposit_fee,
                referral_fee,
                max_validators,
            );
            ix
        };

        let ix = get_instruction(ix);

        let mut account_infos: Vec<AccountInfo> = vec![
            // [w] New StakePool to create.
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Manager
            ctx.accounts.manager.to_account_info(),
            // [] Staker
            ctx.accounts.staker.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Uninitialized validator stake list storage account
            ctx.accounts.validator_list.to_account_info(),
            // [] Reserve stake account must be initialized, have zero balance, and staker / withdrawer authority set to pool withdraw authority.
            ctx.accounts.reserve_stake.to_account_info(),
            // [] Pool token mint. Must have zero supply, owned by withdraw authority.
            ctx.accounts.pool_mint.to_account_info(),
            // [] Pool account to deposit the generated fee for manager.
            ctx.accounts.manager_pool_account.to_account_info(),
            // [] Token program id
            ctx.accounts.token_program.to_account_info(),
        ];

        if ctx.accounts.deposit_authority.is_some() {
            // [] (Optional) Deposit authority that must sign all deposits.
            // Defaults to the program address generated using find_deposit_authority_program_address,
            // making deposits permissionless.
            account_infos.push(
                ctx.accounts
                    .deposit_authority
                    .clone()
                    .unwrap()
                    .to_account_info(),
            );
        }

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
