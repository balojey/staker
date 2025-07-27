import "./App.css"
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { MainLayout } from '@/components/layout/MainLayout';
import { DashboardPage } from '@/pages/DashboardPage';
import { StakingPage } from '@/pages/StakingPage';
import { SwapPage } from '@/pages/SwapPage';
import { useMemo } from 'react';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import('@solana/wallet-adapter-react-ui/styles.css' as any);

function App() {
  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'
  const network = 'devnet';

  // You can also provide a custom RPC endpoint
  const endpoint = useMemo(() => {
    if (network === 'devnet') return 'https://api.devnet.solana.com';
    if (network === 'testnet') return 'https://api.testnet.solana.com';
    return 'https://api.mainnet-beta.solana.com';
  }, [network]);

  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
    ],
    []
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <Router>
            <Routes>
              <Route element={<MainLayout />}>
                <Route path="/" element={<DashboardPage />} />
                <Route path="/stake" element={<StakingPage />} />
                <Route path="/swap" element={<SwapPage />} />
              </Route>
            </Routes>
          </Router>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default App;