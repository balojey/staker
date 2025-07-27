# Section 5: API Specification (On-Chain Program Interface)

## Staking Instructions

The frontend will primarily interact with the following instructions from the Anchor program:

  * `deposit_sol`: To deposit SOL into the pool in exchange for pool tokens (staking).
  * `withdraw_sol`: To burn pool tokens in exchange for SOL (unstaking).
  * `update_stake_pool_balance`: To refresh pool accounting data.
  * [cite\_start]The frontend will also need to read data directly from the `StakePool` and `ValidatorList` accounts to display UI state. [cite: 82-83]

## Architectural Finding: Missing Swap Functionality

The provided `staker.tar` source code is an SPL Stake Pool program. It does not contain the on-chain program for the Token Swap functionality. As per your decision, the MVP has been de-scoped to focus exclusively on the Staking feature.
