use anchor_lang::prelude::*;
use instructions::*;
use utils::state::Fee;
use utils::state::FeeType;
use utils::state::FundingType;
use utils::state::PreferredValidatorType;

mod instructions;
mod utils;

declare_id!("4QbLgtaR4pjTetsyUS7Lqfu8DYQCgKmL5X7ihYgFeTCp");

#[program]
pub mod anchor_spl_stake_pool {

    use super::*;

    pub fn deposit_stake(ctx: Context<DepositStake>) -> Result<()> {
        DepositStake::context_handler(ctx)
    }

    pub fn initialize(
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
        Initialize::context_handler(
            ctx,
            fee.convert(),
            withdrawal_fee.convert(),
            deposit_fee.convert(),
            referral_fee,
            max_validators,
        )
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, lamports_in: u64) -> Result<()> {
        DepositSol::context_handler(ctx, lamports_in)
    }

    pub fn deposit_sol_with_slippage(
        ctx: Context<DepositSolWithSlippage>,
        lamports_in: u64,
        minimum_pool_tokens_out: u64,
    ) -> Result<()> {
        DepositSolWithSlippage::context_handler(ctx, lamports_in, minimum_pool_tokens_out)
    }

    pub fn update_stake_pool_balance(ctx: Context<UpdateStakePoolBalance>) -> Result<()> {
        UpdateStakePoolBalance::context_handler(ctx)
    }

    pub fn add_validator_to_pool(
        ctx: Context<AddValidatorToPool>,
        seed: Option<u32>,
    ) -> Result<()> {
        AddValidatorToPool::context_handler(ctx, seed)
    }

    pub fn remove_validator_from_pool(ctx: Context<RemoveValidatorFromPool>) -> Result<()> {
        RemoveValidatorFromPool::context_handler(ctx)
    }

    pub fn deposit_stake_with_slippage(
        ctx: Context<DepositStakeWithSlippage>,
        minimum_pool_tokens_out: u64,
    ) -> Result<()> {
        DepositStakeWithSlippage::context_handler(ctx, minimum_pool_tokens_out)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, pool_tokens_in: u64) -> Result<()> {
        WithdrawSol::context_handler(ctx, pool_tokens_in)
    }

    pub fn withdraw_sol_with_slippage(
        ctx: Context<WithdrawSolWithSlippage>,
        pool_tokens_in: u64,
        minimum_lamports_out: u64,
    ) -> Result<()> {
        WithdrawSolWithSlippage::context_handler(ctx, pool_tokens_in, minimum_lamports_out)
    }

    pub fn withdraw_stake(ctx: Context<WithdrawStake>, pool_tokens_in: u64) -> Result<()> {
        WithdrawStake::context_handler(ctx, pool_tokens_in)
    }

    pub fn withdraw_stake_with_slippage(
        ctx: Context<WithdrawStakeWithSlippage>,
        pool_tokens_in: u64,
        minimum_lamports_out: u64,
    ) -> Result<()> {
        WithdrawStakeWithSlippage::context_handler(ctx, pool_tokens_in, minimum_lamports_out)
    }

    pub fn increase_validator_stake(
        ctx: Context<IncreaseValidatorStake>,
        lamports: u64,
        transient_stake_seed: u64,
    ) -> Result<()> {
        IncreaseValidatorStake::context_handler(ctx, lamports, transient_stake_seed)
    }

    pub fn increase_validator_stake_with_vote(
        ctx: Context<IncreaseValidatorStakeWithVote>,
        vote_account: Pubkey,
        lamports: u64,
        validator_stake_seed: Option<u32>,
        transient_stake_seed: u64,
    ) -> Result<()> {
        IncreaseValidatorStakeWithVote::context_handler(
            ctx,
            vote_account,
            lamports,
            validator_stake_seed,
            transient_stake_seed,
        )
    }

    pub fn increase_additional_validator_stake(
        ctx: Context<IncreaseAdditionalValidatorStake>,
        lamports: u64,
        transient_stake_seed: u64,
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        IncreaseAdditionalValidatorStake::context_handler(
            ctx,
            lamports,
            transient_stake_seed,
            ephemeral_stake_seed,
        )
    }

    pub fn increase_additional_validator_stake_with_vote(
        ctx: Context<IncreaseAdditionalValidatorStakeWithVote>,
        vote_account: Pubkey,
        lamports: u64,
        validator_stake_seed: Option<u32>,
        transient_stake_seed: u64,
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        IncreaseAdditionalValidatorStakeWithVote::context_handler(
            ctx,
            vote_account,
            lamports,
            validator_stake_seed,
            transient_stake_seed,
            ephemeral_stake_seed,
        )
    }

    pub fn increase_additional_validator_stake_with_list(
        ctx: Context<IncreaseAdditionalValidatorStakeWithList>,
        vote_account: Pubkey,
        lamports: u64,
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        IncreaseAdditionalValidatorStakeWithList::context_handler(
            ctx,
            vote_account,
            lamports,
            ephemeral_stake_seed,
        )
    }

    pub fn decrease_validator_stake_with_reserve(
        ctx: Context<DecreaseValidatorStakeWithReserve>,
        lamports: u64,
        transient_stake_seed: u64,
    ) -> Result<()> {
        DecreaseValidatorStakeWithReserve::context_handler(ctx, lamports, transient_stake_seed)
    }

    pub fn decrease_validator_stake_with_vote(
        ctx: Context<DecreaseValidatorStakeWithVote>,
        vote_account_address: Pubkey,
        lamports: u64,
        validator_stake_seed: Option<u32>,
        transient_stake_seed: u64,
    ) -> Result<()> {
        DecreaseValidatorStakeWithVote::context_handler(
            ctx,
            vote_account_address,
            lamports,
            validator_stake_seed,
            transient_stake_seed,
        )
    }

    pub fn decrease_additional_validator_stake(
        ctx: Context<DecreaseAdditionalValidatorStake>,
        lamports: u64,
        transient_stake_seed: u64,
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        DecreaseAdditionalValidatorStake::context_handler(
            ctx,
            lamports,
            transient_stake_seed,
            ephemeral_stake_seed,
        )
    }

    pub fn decrease_additional_validator_stake_with_list(
        ctx: Context<DecreaseAdditionalValidatorStakeWithList>,
        vote_account: Pubkey,
        lamports: u64,
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        DecreaseAdditionalValidatorStakeWithList::context_handler(
            ctx,
            vote_account,
            lamports,
            ephemeral_stake_seed,
        )
    }

    pub fn decrease_additional_validator_stake_with_vote(
        ctx: Context<DecreaseAdditionalValidatorStakeWithVote>,
        vote_account: Pubkey,
        lamports: u64,
        validator_stake_seed: Option<u32>,
        transient_stake_seed: u64,
        ephemeral_stake_seed: u64,
    ) -> Result<()> {
        DecreaseAdditionalValidatorStakeWithVote::context_handler(
            ctx,
            vote_account,
            lamports,
            validator_stake_seed,
            transient_stake_seed,
            ephemeral_stake_seed,
        )
    }

    pub fn set_preferred_validator(
        ctx: Context<SetPreferredValidator>,
        validator_type: PreferredValidatorType,
        validator_vote_address: Option<Pubkey>,
    ) -> Result<()> {
        SetPreferredValidator::context_handler(
            ctx,
            validator_type.convert(),
            validator_vote_address,
        )
    }

    pub fn cleanup_removed_validator_entries(
        ctx: Context<CleanupRemovedValidatorEntries>,
    ) -> Result<()> {
        CleanupRemovedValidatorEntries::context_handler(ctx)
    }

    pub fn update_validator_list_balance(
        ctx: Context<UpdateValidatorListBalance>,
        start_index: u32,
        no_merge: bool,
    ) -> Result<()> {
        UpdateValidatorListBalance::context_handler(ctx, start_index, no_merge)
    }

    pub fn set_manager(ctx: Context<SetManager>) -> Result<()> {
        SetManager::context_handler(ctx)
    }

    pub fn set_staker(ctx: Context<SetStaker>) -> Result<()> {
        SetStaker::context_handler(ctx)
    }

    pub fn set_fee(ctx: Context<SetFee>, fee: FeeType) -> Result<()> {
        SetFee::context_handler(ctx, fee.convert())
    }

    pub fn set_funding_authority(
        ctx: Context<SetFundingAuthority>,
        funding_type: FundingType,
    ) -> Result<()> {
        SetFundingAuthority::context_handler(ctx, funding_type.convert())
    }

    pub fn create_token_metadata(
        ctx: Context<CreateTokenMetadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        CreateTokenMetadata::context_handler(ctx, name, symbol, uri)
    }

    pub fn update_token_metadata(
        ctx: Context<UpdateTokenMetadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        UpdateTokenMetadata::context_handler(ctx, name, symbol, uri)
    }
}

// DecreaseValidatorStake -> deprecated
// DecreaseValidatorStakeWithReserve
// decrease_validator_stake_with_vote
// decrease_additional_validator_stake
// decrease_additional_validator_stake_with_list
// decrease_additional_validator_stake_with_vote
