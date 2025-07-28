import {
  Connection,
  PublicKey,
  Transaction,
  LAMPORTS_PER_SOL
} from '@solana/web3.js';
import type { WalletContextState } from '@solana/wallet-adapter-react';
// Import the depositSol function directly from the official library
import { depositSol, withdrawSol } from '@solana/spl-stake-pool';

// --- Use the addresses for the specific pool you are targeting ---
// This is the address of the Stake Pool itself, not your wrapper program.
const STAKE_POOL_ADDRESS = new PublicKey('AN7Jf1UZy7oorHP94AC4184JjB4sSFnk37zHuVcMSeSg');

/**
 * Stake SOL using the official @solana/spl-stake-pool library.
 * @param connection Solana RPC connection
 * @param wallet Wallet adapter instance
 * @param amountInSol Amount of SOL to stake
 * @returns Promise that resolves with the transaction signature
 */
export async function stakeSol(
  connection: Connection,
  wallet: WalletContextState | null,
  amountInSol: number
): Promise<string> {
  try {
    if (!wallet?.connected || !wallet?.publicKey || !wallet.sendTransaction) {
      throw new Error('Wallet not connected');
    }

    // const amountInLamports = new BN(amountInSol * LAMPORTS_PER_SOL);
    const info = await connection.getAccountInfo(STAKE_POOL_ADDRESS);
    console.log(info?.owner.toBase58());
    
    // The library's 'depositSol' function correctly creates the entire transaction for you.
    // It automatically finds all the necessary PDAs and provides the correct accounts.
    const { instructions, signers } = await depositSol(
      connection,
      STAKE_POOL_ADDRESS,
      wallet.publicKey,
      amountInSol * LAMPORTS_PER_SOL,
    );

    const transaction = new Transaction().add(...instructions);
    const { blockhash } = await connection.getLatestBlockhash('confirmed');
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = wallet.publicKey;
    console.log('Sending transaction with signers:', signers);
    console.log('Transaction:', transaction);
    
    // The library correctly builds the transaction to call the official SPL Stake Pool program
    try {
      const signature = await wallet.sendTransaction(transaction, connection, { signers });
      await connection.confirmTransaction(signature, 'confirmed');
      return signature;
    } catch (sendError) {
      console.error('Wallet sendTransaction failed:', sendError);
      throw sendError;
    }
  } catch (error) {
    console.error('Error staking SOL:', error);
    throw error;
  }
}

export async function unstakeSol(
  connection: Connection,
  wallet: WalletContextState,
  amountInSol: number
): Promise<string> {
  if (!wallet.publicKey || !wallet.sendTransaction) {
    throw new Error('Wallet not connected!');
  }

  const amountInLamports = amountInSol * LAMPORTS_PER_SOL;
  
  // The library creates the transaction for you
  const { instructions, signers } = await withdrawSol(
    connection,
    STAKE_POOL_ADDRESS,
    wallet.publicKey,
    wallet.publicKey,
    amountInLamports,
  );

  const transaction = new Transaction().add(...instructions);

  const signature = await wallet.sendTransaction(transaction, connection, { signers });
  await connection.confirmTransaction(signature, 'confirmed');
  
  return signature;
}