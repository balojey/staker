# Section 7: Epic 2: Core Staking Functionality

**Epic Goal:** To deliver the core staking feature of the dApp, allowing users to stake tokens, view their position, and withdraw their assets and rewards.

---

**Story 2.1: Display User Token Balances**
* **As a** user, **I want** to see the balances of my relevant SPL tokens, **so that** I know how much I have available to stake.
* **Acceptance Criteria:**
    1.  A component displays the user's balance for the primary stakable token.
    2.  The balance is accurately fetched from the Solana network.
    3.  A loading state is displayed while the balance is being fetched.

---

**Story 2.2: Implement the Token Staking Interface**
* **As a** user, **I want** an input field and a "Stake" button, **so that** I can specify the amount of tokens I wish to stake.
* **Acceptance Criteria:**
    1.  A staking component contains a numerical input field and a "Max" button.
    2.  A "Stake" button is clearly visible.
    3.  The "Stake" button is disabled if the input is invalid or exceeds the user's balance.

---

**Story 2.3: Execute Staking Transaction**
* **As a** user, **I want** to be prompted by my wallet to approve the transaction, **so that** my tokens are securely staked.
* **Acceptance Criteria:**
    1.  Clicking "Stake" prompts the user to sign the correct staking transaction.
    2.  The UI shows a "pending" state while the transaction is being confirmed.
    3.  A success or error notification is displayed upon completion.
    4.  Balances are updated in the UI after a successful transaction.

---

**Story 2.4: Display Staked Position and Rewards**
* **As a** user, **I want** to see my total staked amount and my claimable rewards, **so that** I can track my stake.
* **Acceptance Criteria:**
    1.  A component displays the user's current total staked balance.
    2.  The component displays the amount of rewards available to claim.
    3.  This data is fetched from the staking smart contract.

---

**Story 2.5: Implement Unstaking and Claiming**
* **As a** user, **I want** to be able to unstake my tokens and claim my rewards, **so that** I can retrieve my assets.
* **Acceptance Criteria:**
    1.  "Unstake" and "Claim Rewards" buttons are present.
    2.  Clicking either button initiates the correct on-chain transaction.
    3.  The UI provides clear feedback for both actions.
    4.  All relevant balances are updated in the UI after a successful action.