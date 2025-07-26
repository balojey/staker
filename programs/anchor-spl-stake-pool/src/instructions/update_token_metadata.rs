use {
    crate::utils::{get_instruction, get_pubkey},
    anchor_lang::{prelude::*, solana_program::program::invoke},
    spl_stake_pool::instruction::update_token_metadata,
};

#[derive(Accounts)]
pub struct UpdateTokenMetadata<'info> {
    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool: UncheckedAccount<'info>,

    // [s] Manager
    pub manager: Signer<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool_withdraw_authority: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub pool_mint: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub token_metadata: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub inline_mpl_token_metadata: UncheckedAccount<'info>,

    /// CHECKED: account required and validated on the stake pool program
    pub stake_pool_program: UncheckedAccount<'info>,
}

impl<'info> UpdateTokenMetadata<'info> {
    // Update token metadata for the stake-pool token in the metaplex-token program
    pub fn context_handler(
        ctx: Context<UpdateTokenMetadata>,
        // Token name
        name: String,
        // Token symbol e.g. stkSOL
        symbol: String,
        // URI of the uploaded metadata of the spl-token
        uri: String,
    ) -> Result<()> {
        let ix = update_token_metadata(
            &get_pubkey(ctx.accounts.stake_pool_program.key()),
            &get_pubkey(ctx.accounts.stake_pool.key()),
            &get_pubkey(ctx.accounts.manager.key()),
            &get_pubkey(ctx.accounts.pool_mint.key()),
            name,
            symbol,
            uri,
        );

        let ix = get_instruction(ix);

        let account_infos: Vec<AccountInfo> = vec![
            // [] Stake pool
            ctx.accounts.stake_pool.to_account_info(),
            // [s] Manager
            ctx.accounts.manager.to_account_info(),
            // [] Stake pool withdraw authority
            ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
            // [w] Token metadata account
            ctx.accounts.token_metadata.to_account_info(),
            // [] Metadata program id
            ctx.accounts.inline_mpl_token_metadata.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        Ok(())
    }
}
