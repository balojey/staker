
import { useState, useEffect } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js';

// Manually define program IDs to remove @solana/spl-token dependency
const TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');

// Define a more specific type for the parsed account data
interface ParsedTokenAccountData {
  parsed: {
    info: {
      tokenAmount: {
        uiAmount: number;
      };
    };
  };
}

export const useTokenBalance = (tokenMintAddress: string) => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const fetchBalance = async () => {
      if (!publicKey || !tokenMintAddress) {
        setLoading(false);
        return;
      }

      setLoading(true);
      setError(null);

      try {
        const mintPublicKey = new PublicKey(tokenMintAddress);
        
        // Manually find the ATA to avoid potential browser compatibility issues
        const [ata] = await PublicKey.findProgramAddress(
            [publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintPublicKey.toBuffer()],
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        const accountInfo = await connection.getParsedAccountInfo(ata);

        if (accountInfo.value) {
          const tokenBalance = (accountInfo.value.data as ParsedTokenAccountData).parsed.info.tokenAmount.uiAmount;
          setBalance(tokenBalance);
        } else {
          setBalance(0);
        }
      } catch (e) {
        setError(e as Error);
        setBalance(0);
      } finally {
        setLoading(false);
      }
    };

    fetchBalance();
  }, [publicKey, tokenMintAddress, connection]);

  return { loading, error, balance };
};
