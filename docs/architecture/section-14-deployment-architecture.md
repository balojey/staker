# Section 14: Deployment Architecture

## Deployment Strategy

  * **Frontend:** Deployed via **Vercel**, connected to the main git branch.
  * **Backend (Program):** Deployed manually via the `anchor deploy` command for security.

## CI/CD Pipeline

  * **CI:** GitHub Actions will run tests on every pull request.
  * **CD:** Vercel provides automatic continuous deployment for the frontend.

## Environments

| Environment | Frontend URL | Backend Network | Purpose |
| :--- | :--- | :--- | :--- |
| **Staging** | `*.vercel.app` | Solana Devnet | Pre-production testing. |
| **Production**| `(custom domain)` | Solana Mainnet Beta | Live application. |
