# Section 10: Frontend Architecture

## Component Architecture

  * **Component Organization:** We will adopt a feature-based folder structure inside `src/`.
    ```plaintext
    staker-frontend/
    └── src/
        ├── components/
        │   ├── ui/         # Generic shadcn components (Button, Card, etc.)
        │   └── layout/     # Header, Footer, MainLayout, etc.
        └── features/
            ├── wallet/     # WalletButton, BalanceDisplay, etc.
            └── staking/    # StakingModule, StakingForm, etc.
    ```
  * **Component Template:**
    ```typescript
    import { FC } from 'react';

    interface MyComponentProps {
      title: string;
    }

    export const MyComponent: FC<MyComponentProps> = ({ title }) => { /* ... */ };
    ```

## State Management Architecture

  * **Global State:** We will use **Zustand** for shared application state like wallet connection status and user balances.
  * **Local State:** Standard React hooks (`useState`) will be used for UI-only state (e.g., form inputs).

## Routing Architecture

  * **Library:** We will use `react-router-dom` for client-side routing.
  * **Routes:** `/` (Dashboard/Staking Page), `/*` (404 Not Found).

## Frontend Services Layer

A services layer in `src/services/` will encapsulate all Solana blockchain interactions, separating on-chain logic from UI components.
