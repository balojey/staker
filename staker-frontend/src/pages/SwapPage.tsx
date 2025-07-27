import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletConnect } from '@/components/WalletConnect';
import { ChevronDown, ArrowUpDown } from 'lucide-react';
import { Skeleton } from '@/components/ui/skeleton';

const TOKENS = [
  { symbol: 'SOL', name: 'Solana', balance: '10.5' },
  { symbol: 'USDC', name: 'USD Coin', balance: '1000.00' },
  { symbol: 'BTC', name: 'Bitcoin', balance: '0.1' },
  { symbol: 'ETH', name: 'Ethereum', balance: '2.5' },
];

export function SwapPage() {
  const { connected } = useWallet();
  const [payToken, setPayToken] = useState(TOKENS[0]);
  const [receiveToken, setReceiveToken] = useState(TOKENS[1]);
  const [payAmount, setPayAmount] = useState('');
  const [receiveAmount, setReceiveAmount] = useState('');

  const switchTokens = () => {
    setPayToken(receiveToken);
    setReceiveToken(payToken);
    setPayAmount(receiveAmount);
    setReceiveAmount(payAmount);
  };

  const handleSwap = () => {
    // In a real app, this would initiate the swap transaction
    console.log('Swapping', payAmount, payToken.symbol, 'for', receiveAmount, receiveToken.symbol);
  };

  return (
    <div className="flex flex-col gap-6 max-w-2xl mx-auto w-full">
      <div className="flex flex-col gap-4">
        <h1 className="text-3xl font-bold">Swap Tokens</h1>
        <p className="text-muted-foreground">
          Exchange tokens at the best rates on Solana.
        </p>
      </div>

      {!connected ? (
        <Card className="max-w-md mx-auto">
          <CardHeader>
            <CardTitle>Connect Your Wallet</CardTitle>
            <CardDescription>
              To swap tokens, please connect your Solana wallet.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <WalletConnect />
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <CardTitle>Token Swap</CardTitle>
            <CardDescription>
              Exchange your tokens at competitive rates
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* You Pay Section */}
            <div className="space-y-4">
              <div className="text-sm font-medium">You Pay</div>
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="outline" className="w-[120px] justify-between">
                        <div className="flex items-center gap-2">
                          <div className="bg-muted rounded-full w-5 h-5 flex items-center justify-center">
                            <span className="text-xs font-medium">{payToken.symbol.charAt(0)}</span>
                          </div>
                          <span>{payToken.symbol}</span>
                        </div>
                        <ChevronDown className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start">
                      {TOKENS.map((token) => (
                        <DropdownMenuItem
                          key={token.symbol}
                          onClick={() => setPayToken(token)}
                        >
                          <div className="flex items-center gap-2">
                            <div className="bg-muted rounded-full w-5 h-5 flex items-center justify-center">
                              <span className="text-xs font-medium">{token.symbol.charAt(0)}</span>
                            </div>
                            <div>
                              <div className="font-medium">{token.symbol}</div>
                              <div className="text-xs text-muted-foreground">{token.name}</div>
                            </div>
                          </div>
                        </DropdownMenuItem>
                      ))}
                    </DropdownMenuContent>
                  </DropdownMenu>
                  
                  <div className="flex-1 relative">
                    <Input
                      type="number"
                      placeholder="0.00"
                      value={payAmount}
                      onChange={(e) => setPayAmount(e.target.value)}
                    />
                    <div className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-muted-foreground">
                      {true ? ( // Simulate loading state
                        <Skeleton className="h-4 w-12" />
                      ) : (
                        payToken.balance
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Switch Button */}
            <div className="flex justify-center">
              <Button
                variant="outline"
                size="icon"
                className="rounded-full"
                onClick={switchTokens}
              >
                <ArrowUpDown className="h-4 w-4" />
              </Button>
            </div>

            {/* You Receive Section */}
            <div className="space-y-4">
              <div className="text-sm font-medium">You Receive</div>
              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="outline" className="w-[120px] justify-between">
                        <div className="flex items-center gap-2">
                          <div className="bg-muted rounded-full w-5 h-5 flex items-center justify-center">
                            <span className="text-xs font-medium">{receiveToken.symbol.charAt(0)}</span>
                          </div>
                          <span>{receiveToken.symbol}</span>
                        </div>
                        <ChevronDown className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="start">
                      {TOKENS.map((token) => (
                        <DropdownMenuItem
                          key={token.symbol}
                          onClick={() => setReceiveToken(token)}
                        >
                          <div className="flex items-center gap-2">
                            <div className="bg-muted rounded-full w-5 h-5 flex items-center justify-center">
                              <span className="text-xs font-medium">{token.symbol.charAt(0)}</span>
                            </div>
                            <div>
                              <div className="font-medium">{token.symbol}</div>
                              <div className="text-xs text-muted-foreground">{token.name}</div>
                            </div>
                          </div>
                        </DropdownMenuItem>
                      ))}
                    </DropdownMenuContent>
                  </DropdownMenu>
                  
                  <div className="flex-1 relative">
                    <Input
                      type="number"
                      placeholder="0.00"
                      value={receiveAmount}
                      onChange={(e) => setReceiveAmount(e.target.value)}
                    />
                    <div className="absolute inset-y-0 right-0 flex items-center pr-3 text-sm text-muted-foreground">
                      {true ? ( // Simulate loading state
                        <Skeleton className="h-4 w-12" />
                      ) : (
                        receiveToken.balance
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Exchange Rate */}
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Exchange Rate</span>
              {true ? ( // Simulate loading state
                <Skeleton className="h-4 w-24" />
              ) : (
                <span>1 {payToken.symbol} = 25.5 {receiveToken.symbol}</span>
              )}
            </div>

            {/* Fees */}
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Network Fee</span>
              {true ? ( // Simulate loading state
                <Skeleton className="h-4 w-16" />
              ) : (
                <span>0.00025 SOL</span>
              )}
            </div>

            <Button className="w-full" onClick={handleSwap}>
              Swap Tokens
            </Button>
          </CardContent>
        </Card>
      )}
    </div>
  );
}