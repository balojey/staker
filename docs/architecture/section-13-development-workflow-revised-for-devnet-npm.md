# Section 13: Development Workflow (Revised for Devnet & npm)

## Local Development Setup

  * **Prerequisites:** Node.js, npm, Rust/Cargo, Solana Tool Suite (configured for devnet), Anchor CLI.
  * **Initial Setup:** `git clone ... && cd ... && npm install && solana airdrop 2`
  * **Development Commands (root `package.json`):**
    ```json
    "scripts": {
      "dev:ui": "npm run dev --workspace=staker-frontend",
      "deploy:program": "npm run deploy --workspace=anchor-spl-stake-pool",
      "test:program": "npm test --workspace=anchor-spl-stake-pool"
    }
    ```

## Environment Configuration

  * **`.env` file in `staker-frontend/`:**
    ```bash
    VITE_SOLANA_RPC_ENDPOINT="[https://api.devnet.solana.com](https://api.devnet.solana.com)"
    VITE_STAKE_POOL_PROGRAM_ID="<YOUR_DEVNET_PROGRAM_ID>"
    ```
