import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Skeleton } from '@/components/ui/skeleton';
import { useToast } from '@/hooks/use-toast';
import { Coins, TrendingUp, Wallet } from 'lucide-react';

export function StakingPage() {
  const { connected } = useWallet();
  const { toast } = useToast();
  const [stakeAmount, setStakeAmount] = useState('');
  const [unstakeAmount, setUnstakeAmount] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const handleStake = async () => {
    if (!stakeAmount || parseFloat(stakeAmount) <= 0) {
      toast({
        title: "Invalid Amount",
        description: "Please enter a valid amount to stake",
        variant: "destructive",
      });
      return;
    }

    setIsLoading(true);
    try {
      // Simulate transaction
      await new Promise(resolve => setTimeout(resolve, 2000));
      toast({
        title: "Stake Successful",
        description: `Successfully staked ${stakeAmount} SOL`,
      });
      setStakeAmount('');
    } catch (error) {
      toast({
        title: "Stake Failed",
        description: "Failed to stake tokens. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleUnstake = async () => {
    if (!unstakeAmount || parseFloat(unstakeAmount) <= 0) {
      toast({
        title: "Invalid Amount",
        description: "Please enter a valid amount to unstake",
        variant: "destructive",
      });
      return;
    }

    setIsLoading(true);
    try {
      // Simulate transaction
      await new Promise(resolve => setTimeout(resolve, 2000));
      toast({
        title: "Unstake Successful",
        description: `Successfully unstaked ${unstakeAmount} SOL`,
      });
      setUnstakeAmount('');
    } catch (error) {
      toast({
        title: "Unstake Failed",
        description: "Failed to unstake tokens. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleClaimRewards = async () => {
    setIsLoading(true);
    try {
      // Simulate transaction
      await new Promise(resolve => setTimeout(resolve, 2000));
      toast({
        title: "Rewards Claimed",
        description: "Successfully claimed your staking rewards",
      });
    } catch (error) {
      toast({
        title: "Claim Failed",
        description: "Failed to claim rewards. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };

  if (!connected) {
    return (
      <div className="flex items-center justify-center min-h-[60vh]">
        <Card className="w-full max-w-md">
          <CardHeader className="text-center">
            <CardTitle>Connect Wallet</CardTitle>
            <CardDescription>
              Please connect your wallet to access staking features
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Staking</h1>
        <p className="text-muted-foreground">
          Stake your SOL tokens to earn rewards
        </p>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Available Balance</CardTitle>
            <Wallet className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              <Skeleton className="h-8 w-24" />
            </div>
            <p className="text-xs text-muted-foreground">SOL</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Staked Amount</CardTitle>
            <Coins className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              <Skeleton className="h-8 w-24" />
            </div>
            <p className="text-xs text-muted-foreground">SOL</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Claimable Rewards</CardTitle>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              <Skeleton className="h-8 w-24" />
            </div>
            <p className="text-xs text-muted-foreground">SOL</p>
          </CardContent>
        </Card>
      </div>

      {/* Staking Interface */}
      <Card className="w-full max-w-2xl mx-auto">
        <CardHeader>
          <CardTitle>Stake SOL</CardTitle>
          <CardDescription>
            Earn rewards by staking your SOL tokens
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="stake" className="w-full">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="stake">Stake</TabsTrigger>
              <TabsTrigger value="unstake">Unstake & Claim</TabsTrigger>
            </TabsList>

            <TabsContent value="stake" className="space-y-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Amount to Stake</label>
                <div className="flex space-x-2">
                  <Input
                    type="number"
                    placeholder="0.0"
                    value={stakeAmount}
                    onChange={(e) => setStakeAmount(e.target.value)}
                    className="flex-1"
                  />
                  <Button
                    variant="outline"
                    onClick={() => setStakeAmount('10')} // Mock max amount
                  >
                    Max
                  </Button>
                </div>
                <p className="text-xs text-muted-foreground">
                  Current APY: ~7.5%
                </p>
              </div>

              <Button
                onClick={handleStake}
                disabled={isLoading || !stakeAmount}
                className="w-full"
              >
                {isLoading ? 'Staking...' : 'Stake SOL'}
              </Button>
            </TabsContent>

            <TabsContent value="unstake" className="space-y-4">
              <div className="space-y-4">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Amount to Unstake</label>
                  <div className="flex space-x-2">
                    <Input
                      type="number"
                      placeholder="0.0"
                      value={unstakeAmount}
                      onChange={(e) => setUnstakeAmount(e.target.value)}
                      className="flex-1"
                    />
                    <Button
                      variant="outline"
                      onClick={() => setUnstakeAmount('5')} // Mock max staked amount
                    >
                      Max
                    </Button>
                  </div>
                </div>

                <div className="flex space-x-2">
                  <Button
                    onClick={handleUnstake}
                    disabled={isLoading || !unstakeAmount}
                    variant="outline"
                    className="flex-1"
                  >
                    {isLoading ? 'Unstaking...' : 'Unstake'}
                  </Button>
                  <Button
                    onClick={handleClaimRewards}
                    disabled={isLoading}
                    className="flex-1"
                  >
                    {isLoading ? 'Claiming...' : 'Claim Rewards'}
                  </Button>
                </div>
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}