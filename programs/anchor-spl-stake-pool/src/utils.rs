use {anchor_lang::prelude::*, spl_stake_pool::solana_program::pubkey::Pubkey as StakePubkey};

pub fn get_instruction(
    ix: spl_stake_pool::solana_program::instruction::Instruction,
) -> anchor_lang::solana_program::instruction::Instruction {
    let meta_list: Vec<AccountMeta> = ix
        .accounts
        .iter()
        .map(|account| AccountMeta {
            pubkey: Pubkey::try_from_slice(account.pubkey.as_ref()).unwrap(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    anchor_lang::solana_program::instruction::Instruction {
        program_id: Pubkey::try_from_slice(ix.program_id.as_ref()).unwrap(),
        accounts: meta_list,
        data: ix.data,
    }
}

pub fn get_pubkey(key: Pubkey) -> StakePubkey {
    StakePubkey::try_from_slice(key.as_ref()).unwrap()
}

pub mod data {
    use ::borsh::{self, BorshDeserialize};
    use anyhow::Result;
    use spl_stake_pool::state::ValidatorList;
    use {anchor_lang::prelude::*, spl_stake_pool::state::StakePool};

    pub fn get_stake_pool_data(account_info: &UncheckedAccount) -> Result<StakePool, ProgramError> {
        // Borrow the data from the account
        let account_data = account_info
            .try_borrow_data()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Attempt to deserialize into the StakePool struct
        let stake_pool = StakePool::try_from_slice(&account_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(stake_pool)
    }

    pub fn get_validator_list_data(
        account_info: &UncheckedAccount,
    ) -> Result<ValidatorList, ProgramError> {
        // Borrow the data from the account
        let account_data = account_info
            .try_borrow_data()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Attempt to deserialize into the ValidatorList struct
        let stake_pool = ValidatorList::try_from_slice(&account_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(stake_pool)
    }
}

pub mod state {
    use self::borsh;
    use anchor_lang::prelude::*;

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub enum FundingType {
        /// Sets the stake deposit authority
        StakeDeposit,
        /// Sets the SOL deposit authority
        SolDeposit,
        /// Sets the SOL withdraw authority
        SolWithdraw,
    }

    impl FundingType {
        pub fn convert(self) -> spl_stake_pool::instruction::FundingType {
            match self {
                FundingType::StakeDeposit => spl_stake_pool::instruction::FundingType::StakeDeposit,
                FundingType::SolDeposit => spl_stake_pool::instruction::FundingType::SolDeposit,
                FundingType::SolWithdraw => spl_stake_pool::instruction::FundingType::SolWithdraw,
            }
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub enum PreferredValidatorType {
        /// Set preferred validator for deposits
        Deposit,
        /// Set preferred validator for withdraws
        Withdraw,
    }

    impl PreferredValidatorType {
        pub fn convert(self) -> spl_stake_pool::instruction::PreferredValidatorType {
            match self {
                PreferredValidatorType::Deposit => {
                    spl_stake_pool::instruction::PreferredValidatorType::Deposit
                }
                PreferredValidatorType::Withdraw => {
                    spl_stake_pool::instruction::PreferredValidatorType::Withdraw
                }
            }
        }
    }

    /// The type of fees that can be set on the stake pool
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub enum FeeType {
        /// Referral fees for SOL deposits
        SolReferral(u8),
        /// Referral fees for stake deposits
        StakeReferral(u8),
        /// Management fee paid per epoch
        Epoch(Fee),
        /// Stake withdrawal fee
        StakeWithdrawal(Fee),
        /// Deposit fee for SOL deposits
        SolDeposit(Fee),
        /// Deposit fee for stake deposits
        StakeDeposit(Fee),
        /// SOL withdrawal fee
        SolWithdrawal(Fee),
    }

    impl FeeType {
        pub fn convert(self) -> spl_stake_pool::state::FeeType {
            match self {
                FeeType::SolReferral(data) => spl_stake_pool::state::FeeType::SolReferral(data),
                FeeType::StakeReferral(data) => spl_stake_pool::state::FeeType::StakeReferral(data),
                FeeType::Epoch(Fee {
                    denominator,
                    numerator,
                }) => spl_stake_pool::state::FeeType::Epoch(spl_stake_pool::state::Fee {
                    denominator,
                    numerator,
                }),

                FeeType::StakeWithdrawal(Fee {
                    denominator,
                    numerator,
                }) => spl_stake_pool::state::FeeType::StakeWithdrawal(spl_stake_pool::state::Fee {
                    denominator,
                    numerator,
                }),

                FeeType::SolDeposit(Fee {
                    denominator,
                    numerator,
                }) => spl_stake_pool::state::FeeType::SolDeposit(spl_stake_pool::state::Fee {
                    denominator,
                    numerator,
                }),

                FeeType::StakeDeposit(Fee {
                    denominator,
                    numerator,
                }) => spl_stake_pool::state::FeeType::StakeDeposit(spl_stake_pool::state::Fee {
                    denominator,
                    numerator,
                }),

                FeeType::SolWithdrawal(Fee {
                    denominator,
                    numerator,
                }) => spl_stake_pool::state::FeeType::SolWithdrawal(spl_stake_pool::state::Fee {
                    denominator,
                    numerator,
                }),
            }
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Fee {
        /// denominator of the fee ratio
        pub denominator: u64,
        /// numerator of the fee ratio
        pub numerator: u64,
    }

    impl Fee {
        pub fn convert(self) -> spl_stake_pool::state::Fee {
            spl_stake_pool::state::Fee {
                denominator: self.denominator,
                numerator: self.numerator,
            }
        }
    }
}
