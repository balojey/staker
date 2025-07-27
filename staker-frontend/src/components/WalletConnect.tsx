import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Copy, ExternalLink, Wallet } from 'lucide-react';
import { toast } from 'sonner';

export function WalletConnect() {
  const { connected, disconnect, publicKey } = useWallet();

  const copyToClipboard = () => {
    if (publicKey) {
      navigator.clipboard.writeText(publicKey.toBase58());
      toast.success('Address copied to clipboard');
    }
  };

  const openInExplorer = () => {
    if (publicKey) {
      window.open(`https://explorer.solana.com/address/${publicKey.toBase58()}`, '_blank');
    }
  };

  if (!connected) {
    return (
      <WalletMultiButton
        className="wallet-adapter-button"
        startIcon={<Wallet className="h-4 w-4" />}
        style={{
          fontSize: '0.875rem',
          height: '2.5rem',
          borderRadius: '0.5rem',
          fontWeight: 500,
          paddingLeft: '1rem',
          paddingRight: '1rem',
        }}
      >
        Connect Wallet
      </WalletMultiButton>
    );
  }

  if (!publicKey) return null;

  const address = publicKey.toBase58();
  const shortenedAddress = `${address.slice(0, 4)}...${address.slice(-4)}`;

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" className="font-medium flex items-center gap-2">
          <Wallet className="h-4 w-4" />
          <span className="font-mono text-xs">{shortenedAddress}</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-56">
        <div className="px-2 py-1.5 text-sm font-medium">
          <div className="font-normal text-muted-foreground">Connected as</div>
          <div className="font-mono text-xs truncate">{address}</div>
        </div>
        <DropdownMenuItem onClick={copyToClipboard}>
          <Copy className="mr-2 h-4 w-4" />
          Copy Address
        </DropdownMenuItem>
        <DropdownMenuItem onClick={openInExplorer}>
          <ExternalLink className="mr-2 h-4 w-4" />
          View on Explorer
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => disconnect()}>
          Disconnect
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}