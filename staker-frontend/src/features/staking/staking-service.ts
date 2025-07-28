import {
  Connection,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import type { WalletContextState } from '@solana/wallet-adapter-react';
import { Program, BN } from '@coral-xyz/anchor';
import type { AnchorSplStakePool } from '../../idl/anchor_spl_stake_pool';

// --- Solana Program Constants ---
// Replaced the import from @solana/spl-token with constants
const TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');

// --- Placeholders for your specific addresses ---
const STAKE_POOL_ADDRESS = new PublicKey('5wet8ypoZZNBPStYr988XGMt9uvPWTHNb9Vfin5nvmGU'); // The main stake pool account address
const STAKE_POOL_PROGRAM_ID = new PublicKey('5wet8ypoZZNBPStYr988XGMt9uvPWTHNb9Vfin5nvmGU'); // The Program ID

/**
 * Stake SOL into the stake pool. This version has no dependency on @solana/spl-token.
 * @param connection Solana RPC connection
 * @param wallet Wallet adapter instance
 * @param program An instance of the Anchor Program for your stake pool
 * @param amountInSol Amount of SOL to stake
 * @returns Promise that resolves with the transaction signature
 */
export async function stakeSol(
  connection: Connection,
  wallet: WalletContextState | null,
  program: Program<AnchorSplStakePool>,
  amountInSol: number
): Promise<string> {
  try {
    if (!wallet?.connected || !wallet?.publicKey) {
      throw new Error('Wallet not connected');
    }

    const amountInLamports = new BN(amountInSol * LAMPORTS_PER_SOL);
    const accountInfo = await program.provider.connection.getAccountInfo(STAKE_POOL_ADDRESS);
    if (!accountInfo) {
      throw new Error('Stake pool account not found!');
    }
    const stakePoolInfo = program.coder.accounts.decode('stakePool', accountInfo.data);
    const poolMintAddress = stakePoolInfo.poolMint;
    const managerFeeAccount = stakePoolInfo.managerFeeAccount;
    console.log('Pool Mint Address:', poolMintAddress.toBase58());
    console.log('Manager Fee Account:', managerFeeAccount.toBase58());

    // Find the required PDAs for the instruction
    const [withdrawAuthority] = PublicKey.findProgramAddressSync(
      [STAKE_POOL_ADDRESS.toBuffer(), Buffer.from('withdraw')],
      STAKE_POOL_PROGRAM_ID
    );
    const [reserveStake] = PublicKey.findProgramAddressSync(
      [STAKE_POOL_ADDRESS.toBuffer(), Buffer.from('reserve')],
      STAKE_POOL_PROGRAM_ID
    );

    // Replaced getAssociatedTokenAddressSync with the underlying browser-safe function.
    // This calculates the user's Associated Token Account address for the pool token.
    const [userPoolTokenAccount] = PublicKey.findProgramAddressSync(
        [
            wallet.publicKey.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            poolMintAddress,
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const transaction = await program.methods
      .depositSol(amountInLamports)
      .accounts({
        stakePool: STAKE_POOL_ADDRESS,
        reserveStakeAccount: reserveStake,
        lamportsFrom: wallet.publicKey,
        poolTokensTo: userPoolTokenAccount,
        managerFeeAccount: managerFeeAccount,
        referrerPoolTokensAccount: userPoolTokenAccount,
        poolMint: poolMintAddress,
        stakePoolWithdrawAuthority: withdrawAuthority,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .transaction();

    const signature = await wallet.sendTransaction(transaction, connection);

    await connection.confirmTransaction(signature, 'confirmed');

    return signature;
  } catch (error) {
    console.error('Error staking SOL:', error);
    throw error;
  }
}