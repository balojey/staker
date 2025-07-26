# 🎯 SPL Stake Pool — Template

**Type:** Anchor

---

## 📘 Use Case

This template implements a full-featured interface to the official SPL Stake Pool program using Anchor. It allows users to stake SOL or stake accounts and receive pool tokens in return. The pool manager can dynamically add or remove validators, manage fees, rebalance validator stakes, and interact with on-chain token metadata. Ideal for building liquid staking protocols or validator management platforms on Solana.

---

## 🧱 Data Structure

| Account         | Description                                                     |
| --------------- | --------------------------------------------------------------- |
| `StakePool`     | Holds pool config, authorities, fees, validator list, and mint. |
| `ValidatorList` | Stores all validator stake entries registered in the pool.      |
| `StakeAccount`  | User or validator-specific stake accounts managed by the pool.  |
| `PoolMint`      | SPL Token mint representing the stake pool token.               |
| `TokenAccount`  | Associated Token Accounts for holding pool tokens.              |

---

## 🧾 Instructions

| Name                                 | Description                                                            |
| ------------------------------------ | ---------------------------------------------------------------------- |
| `initialize`                         | Initializes the stake pool with fees, max validators, and authorities. |
| `deposit_sol`                        | Allows a user to deposit SOL and receive pool tokens.                  |
| `deposit_stake`                      | Deposits a user-provided stake account into the pool.                  |
| `withdraw_sol`                       | Withdraws SOL in exchange for burning pool tokens.                     |
| `withdraw_stake`                     | Withdraws stake accounts instead of SOL.                               |
| `update_stake_pool_balance`          | Updates internal accounting of validator balances.                     |
| `add_validator_to_pool`              | Adds a validator vote account to the stake pool.                       |
| `remove_validator_from_pool`         | Removes a validator from the pool.                                     |
| `increase_validator_stake`           | Delegates additional stake to a validator.                             |
| `decrease_validator_stake_with_vote` | Deactivates and withdraws stake from a validator.                      |
| `set_preferred_validator`            | Sets a preferred validator for deposits/withdrawals.                   |
| `update_validator_list_balance`      | Refreshes balances and optionally merges transient stake.              |
| `set_manager`, `set_staker`          | Admin functions to update pool manager or staker.                      |
| `set_fee`, `set_funding_authority`   | Admin functions to update fees or funding permissions.                 |
| `create_token_metadata`              | Creates on-chain metadata for the stake pool token.                    |
| `update_token_metadata`              | Updates the name, symbol, or URI of the pool token.                    |
