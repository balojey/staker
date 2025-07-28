import React from 'react';
import { useSolBalance } from './useSolBalance';
import { Skeleton } from '@/components/ui/skeleton';

interface BalanceDisplayProps {
  label?: string;
  onRefresh?: () => void;
}

const BalanceDisplay: React.FC<BalanceDisplayProps> = ({ label = 'Balance', onRefresh }) => {
  const { balance, loading, refresh } = useSolBalance();

  const handleRefresh = () => {
    refresh();
    onRefresh?.();
  };

  React.useEffect(() => {
    // Refresh balance when component mounts
    handleRefresh();
  }, []);

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