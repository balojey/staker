import React, { useEffect } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { useBalanceStore } from '@/stores/useBalanceStore';
import { Skeleton } from '@/components/ui/skeleton';

interface BalanceDisplayProps {
  label?: string;
}

const BalanceDisplay: React.FC<BalanceDisplayProps> = ({ label = 'Balance' }) => {
  // Get state and actions from the global Zustand store
  const { solBalance, isLoading, fetchBalances } = useBalanceStore();
  const { connection } = useConnection();
  const { publicKey } = useWallet();

  // Fetch balances when the component mounts or the wallet changes
  useEffect(() => {
    if (publicKey) {
      fetchBalances(connection, publicKey);
    }
  }, [publicKey, connection, fetchBalances]);

  if (isLoading) {
    return <Skeleton className="h-6 w-20" />;
  }

  return (
    <div className="flex flex-col items-end">
      <span className="text-sm text-muted-foreground">{label}</span>
      <span className="text-lg font-semibold">
        {solBalance.toFixed(4)} SOL
      </span>
    </div>
  );
};

export default BalanceDisplay;