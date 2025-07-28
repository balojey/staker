// src/stores/useBalanceStore.ts
import { create } from 'zustand';
import { Connection, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
// The rest of your imports...

interface BalanceState {
  solBalance: number;
  stakedSolBalance: number;
  isLoading: boolean; // <-- Added loading state
  fetchBalances: (connection: Connection, publicKey: PublicKey) => Promise<void>;
}

export const useBalanceStore = create<BalanceState>((set) => ({
  solBalance: 0,
  stakedSolBalance: 0,
  isLoading: true, // Start in loading state
  fetchBalances: async (connection: Connection, publicKey: PublicKey) => {
    set({ isLoading: true });
    try {
      // Fetch SOL balance
      const lamports = await connection.getBalance(publicKey);
      set({ solBalance: lamports / LAMPORTS_PER_SOL });

      // In a real app, you would also fetch the staked balance here
      // set({ stakedSolBalance: ... });

    } catch (error) {
      console.error("Failed to fetch balances:", error);
      set({ solBalance: 0, stakedSolBalance: 0 });
    } finally {
      set({ isLoading: false }); // <-- Ensure loading is always set to false
    }
  },
}));