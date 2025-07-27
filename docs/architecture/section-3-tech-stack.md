# Section 3: Tech Stack

## Technology Stack Table

| Category | Technology | Version | Purpose | Rationale |
| :--- | :--- | :--- | :--- | :--- |
| **Frontend Language** | TypeScript | `~5.4.5` | Adds static typing to JavaScript for robustness. | Industry standard for React development; prevents common errors. |
| **Frontend Framework**| React | `~18.3.1` | The core UI library for building components. | User-specified and a leading choice for dApp development. |
| **UI Component Library**| shadcn/ui | `~0.8.0` | Provides unstyled, accessible components. | User-specified; offers full control over styling. |
| **State Management** | Zustand | `~4.5.2` | Lightweight global state management. | Simpler than Redux for this app's scope, minimal boilerplate. |
| **Backend Language** | Rust | `~1.79.0` | Language for the on-chain programs. | Existing backend technology; high performance and safety. |
| **Backend Framework** | Anchor | `~0.30.1` | Solana smart contract development framework. | Existing backend framework; simplifies Solana development. |
| **API Style** | JSON-RPC | `N/A` | Protocol for interacting with Solana nodes. | Standard method for all blockchain communication. |
| **Database** | Solana Blockchain | `N/A` | The decentralized ledger for all state. | Core of the dApp; provides a single source of truth. |
| **Authentication** | Solana Wallet Adapter| `~1.2.1` | Securely connects the dApp to user wallets. | Standard, secure way to handle authentication in Solana dApps. |
| **Frontend Testing** | Vitest + RTL | `~1.6.0` | Unit/Integration testing framework. | Native to Vite for fast performance; React Testing Library (RTL) is standard. |
| **E2E Testing** | Playwright | `~1.45.0` | End-to-end testing of user flows. | Modern, powerful, and reliable for testing browser interactions. |
| **Build Tool/Bundler**| Vite | `~5.3.1` | The development server and build tool. | Identified in generated code; known for excellent performance. |
| **CI/CD** | Vercel CI | `N/A` | Built-in continuous integration and deployment. | Comes with the recommended Vercel hosting platform. |
| **Monitoring** | Vercel Analytics | `N/A` | User and performance analytics. | Comes with the recommended Vercel hosting platform. |
| **CSS Framework** | Tailwind CSS | `~3.4.4` | Utility-first CSS framework for styling. | User-specified; excellent for rapid, custom UI development. |
| **Package Manager** | npm | `~10.8.1` | Package manager and monorepo orchestrator. | User-specified preference. |
