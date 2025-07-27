import { useState, useEffect } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

export const useSolBalance = () => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!connection || !publicKey) {
      setLoading(false);
      return;
    }

    const getBalance = async () => {
      setLoading(true);
      try {
        const lamports = await connection.getBalance(publicKey);
        setBalance(lamports / LAMPORTS_PER_SOL);
      } catch (error) {
        console.error("Failed to get SOL balance", error);
        setBalance(null);
      } finally {
        setLoading(false);
      }
    };

    getBalance();
  }, [connection, publicKey]);

  return { balance, loading };
};
