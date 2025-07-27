# Section 6: Components

## Component List

  * **`WalletProvider` (Wrapper)**
      * **Responsibility:** Manages the global state of the wallet connection using the Solana Wallet Adapter.
  * **`WalletButton`**
      * **Responsibility:** The main UI element for connecting and disconnecting a wallet.
  * **`BalanceDisplay`**
      * **Responsibility:** A reusable component that fetches and displays the user's balance for a given SPL token.
  * **`StakingModule`**
      * **Responsibility:** The primary component for all staking-related actions (staking, unstaking, claiming rewards).
  * **`NotificationProvider` (Wrapper)**
      * **Responsibility:** A global provider for displaying toast notifications (e.g., "Transaction successful").
