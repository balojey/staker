import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletConnect } from '@/components/WalletConnect';

export function DashboardPage() {
  const { connected } = useWallet();

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-4">
        <h1 className="text-3xl font-bold">Dashboard</h1>
        <p className="text-muted-foreground">
          Welcome to Staker! Connect your wallet to get started.
        </p>
      </div>

      {!connected ? (
        <Card className="max-w-md">
          <CardHeader>
            <CardTitle>Connect Your Wallet</CardTitle>
            <CardDescription>
              To access all features, please connect your Solana wallet.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <WalletConnect />
          </CardContent>
        </Card>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <Card>
            <CardHeader>
              <CardTitle>Total Staked</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">0.00 SOL</div>
              <p className="text-sm text-muted-foreground">+0% from last month</p>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle>Rewards Earned</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">0.00 SOL</div>
              <p className="text-sm text-muted-foreground">Available to claim</p>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle>APY</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">8.5%</div>
              <p className="text-sm text-muted-foreground">Annual percentage yield</p>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}