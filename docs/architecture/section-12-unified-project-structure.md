# Section 12: Unified Project Structure

```plaintext
staker-dapp/
├── staker-frontend/          # The React (Vite) frontend application
│   ├── src/
│   └── package.json
├── programs/
│   └── anchor-spl-stake-pool/ # The existing on-chain Anchor program
├── .gitignore
├── package.json              # Root package.json with npm workspaces
└── README.md
```
