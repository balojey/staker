# Section 4: Data Models

## Token

  * **Purpose:** Represents an SPL token, including its metadata. This is a foundational model needed for the staking feature.
  * **TypeScript Interface:**

<!-- end list -->

```typescript
export interface Token {
  address: string; // The token's mint address
  symbol: string;
  name: string;
  decimals: number;
  logoURI?: string; // Optional URL for the token's image
}
```

## UserStakedPosition

  * **Purpose:** Represents a user's complete position in the stake pool, containing all the necessary information to be displayed on the staking page.
  * **TypeScript Interface:**

<!-- end list -->

```typescript
export interface UserStakedPosition {
  stakedPoolTokens: number; // The amount of pool tokens the user holds
  stakedInSol: number;      // The SOL equivalent value of their staked tokens
  claimableRewards: number; // The amount of rewards earned in pool tokens
  rewardsInSol: number;     // The SOL equivalent value of their rewards
}
```
