# Section 11: Backend Architecture

The backend is the existing on-chain Anchor program. Its architecture is defined by the Anchor framework, with public instructions in `lib.rs` serving as the API and on-chain accounts serving as the data layer. Authorization is managed via the Solana runtime's requirement for transaction signatures.
