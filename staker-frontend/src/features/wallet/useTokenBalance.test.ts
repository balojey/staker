
import { renderHook, waitFor } from '@testing-library/react';
import { useTokenBalance } from './useTokenBalance';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js';
import { describe, it, expect, beforeEach, vi } from 'vitest';

// Mocks
vi.mock('@solana/wallet-adapter-react', () => ({
  useConnection: vi.fn(),
  useWallet: vi.fn(),
}));

const mockGetParsedAccountInfo = vi.fn();

const mockPublicKey = new PublicKey('11111111111111111111111111111111');

describe('useTokenBalance', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (useConnection as vi.Mock).mockReturnValue({ connection: { getParsedAccountInfo: mockGetParsedAccountInfo } });
  });

  it('should return loading state initially', () => {
    (useWallet as vi.Mock).mockReturnValue({ publicKey: mockPublicKey });
    const { result } = renderHook(() => useTokenBalance('some_token_mint'));
    expect(result.current.loading).toBe(true);
  });

  it('should return balance when account exists', async () => {
    (useWallet as vi.Mock).mockReturnValue({ publicKey: mockPublicKey });
    mockGetParsedAccountInfo.mockResolvedValue({
      value: {
        data: {
          parsed: {
            info: {
              tokenAmount: {
                uiAmount: 123,
              },
            },
          },
        },
      },
    });

    const { result } = renderHook(() => useTokenBalance('some_token_mint'));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
      expect(result.current.balance).toBe(123);
    });
  });

  it('should return 0 when account does not exist', async () => {
    (useWallet as vi.Mock).mockReturnValue({ publicKey: mockPublicKey });
    mockGetParsedAccountInfo.mockResolvedValue({ value: null });

    const { result } = renderHook(() => useTokenBalance('some_token_mint'));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
      expect(result.current.balance).toBe(0);
    });
  });

  it('should return error when fetch fails', async () => {
    (useWallet as vi.Mock).mockReturnValue({ publicKey: mockPublicKey });
    const error = new Error('Failed to fetch');
    mockGetParsedAccountInfo.mockRejectedValue(error);

    const { result } = renderHook(() => useTokenBalance('some_token_mint'));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBe(error);
      expect(result.current.balance).toBe(0);
    });
  });
});
