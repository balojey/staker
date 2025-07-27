import React from 'react';
import { useTokenBalance } from './useTokenBalance';
import { Skeleton } from '@/components/ui/skeleton';

interface BalanceDisplayProps {
  tokenMintAddress: string;
}

const BalanceDisplay: React.FC<BalanceDisplayProps> = ({ tokenMintAddress }) => {
  const { balance, loading } = useTokenBalance(tokenMintAddress);

  if (loading) {
    return <Skeleton className="h-8 w-24" />;
  }

  return (
    <div>
      <p>Balance: {balance !== null ? balance : '0'}</p>
    </div>
  );
};

export default BalanceDisplay;