# Section 2: High Level Architecture

## Technical Summary
The system will be a decentralized application (dApp) composed of a client-side React frontend and existing on-chain Solana smart contracts. The frontend will be deployed globally on a JAMstack hosting platform like Vercel for optimal performance and scalability. Users will interact with the application through their web browser, connecting their self-custody wallets (e.g., Phantom, Solflare) to sign and send transactions directly to the Solana network. The project's code will be managed in a Monorepo to facilitate code sharing and streamlined development.

## Platform and Infrastructure Choice
* **Platform:**
    * **Frontend Hosting:** **Vercel** is recommended for its seamless integration with modern React frameworks (like Vite), automatic CI/CD, global CDN, and serverless functions for any future needs.
    * **Backend & State:** The **Solana Blockchain** will serve as our backend. All application state and business logic are managed by the on-chain smart contracts.
* **Key Services:**
    * **Solana RPC Node:** To read on-chain data and submit transactions.
    * **Solana Wallet Adapter:** To securely connect with users' wallets.

## Repository Structure
* **Structure:** **Monorepo**.
* **Monorepo Tool:** We will use `npm workspaces` to manage the packages.
* **Package Organization:** The monorepo will contain the `staker-frontend` app and the `programs` directory at the root level.

## High Level Architecture Diagram
```mermaid
graph TD
    subgraph User's Browser
        A[React Frontend on Vercel] --> B[Solana Wallet Adapter];
        B --> C[User's Wallet (Phantom/Solflare)];
    end

    subgraph Solana Network
        D[RPC Node];
        E[SPL Stake Pool Programs];
    end

    C -- Signs Transaction --> A;
    A -- Submits Signed Transaction --> D;
    A -- Reads On-chain Data --> D;
    D -- Interacts with --> E;
```

## Architectural Patterns

  * **JAMstack Architecture:** The frontend will be a static/server-rendered site hosted on a global CDN, with dynamic functionality handled by client-side JavaScript interacting with the Solana network. *Rationale: This provides excellent performance, high security, and lower cost.*
  * **Component-Based UI:** We will use React's component model to build a reusable and maintainable user interface, leveraging `shadcn/ui`. *Rationale: This is standard for modern web development and aligns with the chosen tech stack.*
  * **Non-Custodial Wallet Interaction:** The application will never have access to user private keys. All transactions are created client-side and signed by the user's own wallet extension. *Rationale: This is the fundamental security model for dApps and ensures user funds are always in their control.*
