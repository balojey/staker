import BalanceDisplay from '@/features/wallet/BalanceDisplay';
import { useState, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useWallet } from '@solana/wallet-adapter-react';
import { useConnection } from '@solana/wallet-adapter-react';
import { WalletConnect } from '@/components/WalletConnect';
import { Coins, TrendingUp, Wallet } from 'lucide-react';
import { stakeSol } from '@/features/staking/staking-service';
import { toast } from 'sonner';

export function StakingPage() {
  const wallet = useWallet();
  const { connection } = useConnection();
  const [amount, setAmount] = useState('');
  const [activeTab, setActiveTab] = useState('stake');
  const [isStaking, setIsStaking] = useState(false);
  const balanceRefreshRef = useRef<() => void>(() => {});

  const handleMax = () => {
    // In a real app, this would be the user's balance
    setAmount('100');
  };

  const handleStake = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      toast.error('Please enter a valid amount');
      return;
    }

    setIsStaking(true);
    try {
      const signature = await stakeSol(connection, wallet, parseFloat(amount));
      toast.success(`Staking successful! Transaction: ${signature.substring(0, 10)}...`);
      
      // Refresh balances after successful transaction
      balanceRefreshRef.current();
    } catch (error) {
      console.error('Staking failed:', error);
      toast.error(`Staking failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsStaking(false);
    }
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
      <div className="flex flex-col gap-2">
        <h1 className="text-3xl font-bold flex items-center gap-2">
          <Coins className="h-8 w-8 text-primary" />
          Stake Tokens
        </h1>
        <p className="text-muted-foreground">
          Stake your SOL tokens to earn rewards over time.
        </p>
      </div>

      {!wallet.connected ? (
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
            <CardTitle className="flex items-center gap-2">
              <TrendingUp className="h-5 w-5" />
              Staking Pool
            </CardTitle>
            <CardDescription>
              Stake SOL tokens to earn rewards
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="stake" className='text-white'>Stake</TabsTrigger>
                <TabsTrigger value="unstake" className='text-white'>Unstake/Claim</TabsTrigger>
              </TabsList>
              
              <TabsContent value="stake" className="space-y-6 pt-4">
                <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
                  <div className="flex items-center gap-3">
                    <Wallet className="h-5 w-5 text-muted-foreground" />
                    <div>
                      <h3 className="font-medium">Your Balance</h3>
                      <p className="text-sm text-muted-foreground">Available to stake</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <BalanceDisplay />
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
                      className="h-6 px-2 text-xs text-white"
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
                      className="pr-16 py-6 text-lg"
                    />
                    <div className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-muted-foreground">
                      SOL
                    </div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between text-sm p-4 bg-muted rounded-lg">
                  <span className="text-muted-foreground flex items-center gap-2">
                    <TrendingUp className="h-4 w-4" />
                    Annual Percentage Yield
                  </span>
                  <span className="font-bold text-lg">8.5%</span>
                </div>
                
                <Button className="w-full py-6 text-lg" onClick={handleStake} disabled={isStaking}>
                  {isStaking ? (
                    <>
                      <span className="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"></span>
                      Processing...
                    </>
                  ) : (
                    'Stake Tokens'
                  )}
                </Button>
              </TabsContent>
              
              <TabsContent value="unstake" className="space-y-6 pt-4">
                <div className="space-y-4">
                  <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
                    <div>
                      <h3 className="font-medium">Staked Balance</h3>
                      <p className="text-sm text-muted-foreground">Currently staked</p>
                    </div>
                    <div className="text-right">
                      <p className="font-bold text-lg">50.25 SOL</p>
                    </div>
                  </div>
                  
                  <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
                    <div>
                      <h3 className="font-medium">Rewards</h3>
                      <p className="text-sm text-muted-foreground">Available to claim</p>
                    </div>
                    <div className="text-right">
                      <p className="font-bold text-lg">2.15 SOL</p>
                    </div>
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
                      className="pr-16 py-6 text-lg"
                    />
                    <div className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-muted-foreground">
                      SOL
                    </div>
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <Button className="flex-1 py-6 text-lg" onClick={handleUnstake}>
                    Unstake
                  </Button>
                  <Button className="flex-1 py-6 text-lg" variant="secondary" onClick={handleClaim}>
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