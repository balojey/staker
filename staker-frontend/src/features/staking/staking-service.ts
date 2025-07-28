import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import type { WalletContextState } from '@solana/wallet-adapter-react';
import { BN } from '@coral-xyz/anchor';
import { struct, u64 } from '@coral-xyz/borsh'; // Correct import
import { sha256 } from 'js-sha256';

// --- Solana Program Constants ---
const TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');

// --- Your specific addresses ---
const STAKE_POOL_ADDRESS = new PublicKey('4QbLgtaR4pjTetsyUS7Lqfu8DYQCgKmL5X7ihYgFeTCp');
export const STAKE_POOL_PROGRAM_ID = new PublicKey('4QbLgtaR4pjTetsyUS7Lqfu8DYQCgKmL5X7ihYgFeTCp');
const POOL_TOKEN_MINT = new PublicKey('4QbLgtaR4pjTetsyUS7Lqfu8DYQCgKmL5X7ihYgFeTCp');
const MANAGER_FEE_ACCOUNT = new PublicKey('4QbLgtaR4pjTetsyUS7Lqfu8DYQCgKmL5X7ihYgFeTCp');

/**
 * Manually constructs and sends a 'depositSol' transaction.
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

    // --- 1. Manually build the instruction data buffer ---
    
    const instructionName = "global:deposit_sol";
    const discriminator = Buffer.from(sha256.digest(instructionName)).slice(0, 8);
    const instructionLayout = struct([u64('amount')]);
    const amountInLamports = new BN(amountInSol * LAMPORTS_PER_SOL);
    const dataBuffer = Buffer.alloc(1000); 
    instructionLayout.encode({ amount: amountInLamports }, dataBuffer);
    const instructionData = dataBuffer.slice(0, instructionLayout.span);
    const finalDataBuffer = Buffer.concat([discriminator, instructionData]);

    // --- 2. Manually define the account list ---
    
    const [withdrawAuthority] = PublicKey.findProgramAddressSync(
      [STAKE_POOL_ADDRESS.toBuffer(), Buffer.from('withdraw')],
      STAKE_POOL_PROGRAM_ID
    );
    const [reserveStake] = PublicKey.findProgramAddressSync(
      [STAKE_POOL_ADDRESS.toBuffer(), Buffer.from('reserve')],
      STAKE_POOL_PROGRAM_ID
    );
    const [userPoolTokenAccount] = PublicKey.findProgramAddressSync(
        [wallet.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), POOL_TOKEN_MINT.toBuffer()],
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // ** FIX: Calculate the default SOL deposit authority PDA **
    const [solDepositAuthority] = PublicKey.findProgramAddressSync(
        [STAKE_POOL_ADDRESS.toBuffer(), Buffer.from('sol-deposit-authority')],
        STAKE_POOL_PROGRAM_ID
    );

    const keys = [
      { pubkey: STAKE_POOL_ADDRESS, isSigner: false, isWritable: true },
      { pubkey: withdrawAuthority, isSigner: false, isWritable: false },
      { pubkey: reserveStake, isSigner: false, isWritable: true },
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: userPoolTokenAccount, isSigner: false, isWritable: true },
      { pubkey: MANAGER_FEE_ACCOUNT, isSigner: false, isWritable: true },
      { pubkey: userPoolTokenAccount, isSigner: false, isWritable: true }, // Referrer account
      { pubkey: POOL_TOKEN_MINT, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      // ** FIX: Add the missing account to the list **
      { pubkey: solDepositAuthority, isSigner: false, isWritable: false },
    ];
    
    // --- 3. Create and send the transaction ---
    const instruction = new TransactionInstruction({
        keys: keys,
        programId: STAKE_POOL_PROGRAM_ID,
        data: finalDataBuffer,
    });
    
    const transaction = new Transaction().add(instruction);
    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash('confirmed');
    const { publicKey, signTransaction } = wallet
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = publicKey;
    const signedTransaction = signTransaction ? await signTransaction(transaction) : transaction;

    const signature = await connection.sendRawTransaction(
      signedTransaction.serialize(),
      { skipPreflight: false, preflightCommitment: 'confirmed' }
    );

    await connection.confirmTransaction(
      { signature, blockhash, lastValidBlockHeight },
      'confirmed'
    );
    // const signature = await wallet.sendTransaction(transaction, connection);
    // await connection.confirmTransaction(signature, 'confirmed');

    console.log('Transaction confirmed:', signature);
    return signature;

  } catch (error) {
    console.error('Error staking SOL:', error);
    throw error;
  }
}