import React from 'react';
import { useTokenBalance } from './useTokenBalance';
import { Skeleton } from '@/components/ui/skeleton';

interface BalanceDisplayProps {
  tokenMintAddress: string;
  label?: string;
}

const BalanceDisplay: React.FC<BalanceDisplayProps> = ({ tokenMintAddress, label = 'Balance' }) => {
  const { balance, loading } = useTokenBalance(tokenMintAddress);

  if (loading) {
    return <Skeleton className="h-6 w-20" />;
  }

  return (
    <div className="flex flex-col items-end">
      <span className="text-sm text-muted-foreground">{label}</span>
      <span className="text-lg font-semibold">
        {balance !== null ? balance.toFixed(4) : '0.0000'} SOL
      </span>
    </div>
  );
};

export default BalanceDisplay;