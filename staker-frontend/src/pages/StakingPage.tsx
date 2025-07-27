import BalanceDisplay from '@/features/wallet/BalanceDisplay';
import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletConnect } from '@/components/WalletConnect';

// TODO: Replace with actual token mint address from config
const STAKABLE_TOKEN_MINT = 'So11111111111111111111111111111111111111112'; 

export function StakingPage() {
  const { connected } = useWallet();
  const [amount, setAmount] = useState('');
  const [activeTab, setActiveTab] = useState('stake');

  const handleMax = () => {
    // In a real app, this would be the user's balance
    setAmount('100');
  };

  const handleStake = () => {
    // In a real app, this would call the staking contract
    console.log('Staking', amount);
  };

  const handleUnstake = () => {
    // In a real app, this would call the unstaking contract
    console.log('Unstaking', amount);
  };

  const handleClaim = () => {
    // In a real app, this would claim rewards
    console.log('Claiming rewards');
  };

  return (
    <div className="flex flex-col gap-6 max-w-2xl mx-auto w-full">
      <div className="flex flex-col gap-4">
        <h1 className="text-3xl font-bold">Stake Tokens</h1>
        <p className="text-muted-foreground">
          Stake your SOL tokens to earn rewards over time.
        </p>
      </div>

      {!connected ? (
        <Card className="max-w-md mx-auto">
          <CardHeader>
            <CardTitle>Connect Your Wallet</CardTitle>
            <CardDescription>
              To stake tokens, please connect your Solana wallet.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <WalletConnect />
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <CardTitle>Staking Pool</CardTitle>
            <CardDescription>
              Stake SOL tokens to earn rewards
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="stake">Stake</TabsTrigger>
                <TabsTrigger value="unstake">Unstake/Claim</TabsTrigger>
              </TabsList>
              
              <TabsContent value="stake" className="space-y-4 pt-4">
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-medium">Your Balance</h3>
                    <p className="text-sm text-muted-foreground">Available to stake</p>
                  </div>
                  <div className="text-right">
                    <BalanceDisplay tokenMintAddress={STAKABLE_TOKEN_MINT} />
                  </div>
                </div>
                
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <label htmlFor="amount" className="text-sm font-medium">
                      Amount
                    </label>
                    <Button 
                      type="button" 
                      variant="ghost" 
                      size="sm" 
                      className="h-6 px-2 text-xs"
                      onClick={handleMax}
                    >
                      Max
                    </Button>
                  </div>
                  <div className="relative">
                    <Input
                      id="amount"
                      type="number"
                      placeholder="0.00"
                      value={amount}
                      onChange={(e) => setAmount(e.target.value)}
                      className="pr-16"
                    />
                    <div className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-muted-foreground">
                      SOL
                    </div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">APY</span>
                  <span className="font-medium">8.5%</span>
                </div>
                
                <Button className="w-full" onClick={handleStake}>
                  Stake Tokens
                </Button>
              </TabsContent>
              
              <TabsContent value="unstake" className="space-y-4 pt-4">
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-medium">Staked Balance</h3>
                    <p className="text-sm text-muted-foreground">Currently staked</p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium">50.25 SOL</p>
                  </div>
                </div>
                
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-medium">Rewards</h3>
                    <p className="text-sm text-muted-foreground">Available to claim</p>
                  </div>
                  <div className="text-right">
                    <p className="font-medium">2.15 SOL</p>
                  </div>
                </div>
                
                <div className="space-y-2">
                  <label htmlFor="unstake-amount" className="text-sm font-medium">
                    Amount to Unstake
                  </label>
                  <div className="relative">
                    <Input
                      id="unstake-amount"
                      type="number"
                      placeholder="0.00"
                      value={amount}
                      onChange={(e) => setAmount(e.target.value)}
                      className="pr-16"
                    />
                    <div className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-muted-foreground">
                      SOL
                    </div>
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <Button className="flex-1" onClick={handleUnstake}>
                    Unstake
                  </Button>
                  <Button className="flex-1" variant="secondary" onClick={handleClaim}>
                    Claim Rewards
                  </Button>
                </div>
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>
      )}
    </div>
  );
}