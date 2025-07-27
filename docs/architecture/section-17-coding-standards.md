# Section 17: Coding Standards

## Critical Rules for AI Agents

  * **Service Layer Abstraction:** All on-chain interactions must be handled within the `src/services/` layer.
  * **Non-Custodial Principle:** No private keys or seed phrases should ever be handled by the frontend code.
  * **State Immutability:** Global state must be treated as immutable.

## Naming Conventions

| Element | Convention | Example |
| :--- | :--- | :--- |
| **React Components** | `PascalCase` | `StakingModule.tsx` |
| **React Hooks** | `use` + `camelCase` | `useStakingRewards.ts`|
| **On-Chain Instructions**| `snake_case` | `deposit_sol` |
