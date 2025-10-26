"use client";

import type { WalletError } from "@solana/wallet-adapter-base";
import {
    ConnectionProvider,
    useConnection,
    useWallet,
    WalletProvider,
} from "@solana/wallet-adapter-react";
import type { AnchorWallet } from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import dynamic from "next/dynamic";
import { ReactNode, useCallback, useMemo } from "react";
import "@solana/wallet-adapter-react-ui/styles.css";
import { AnchorProvider } from "@coral-xyz/anchor";
import { useCluster } from "./ClusterProvider";

export const WalletButton = dynamic(
    async () =>
        (await import("@solana/wallet-adapter-react-ui")).WalletMultiButton,
    {
        ssr: false,
    }
);

export function SolanaProvider({ children }: { children: ReactNode }) {
    const { cluster } = useCluster();
    const endpoint = useMemo(() => cluster.endpoint, [cluster]);
    const onError = useCallback((error: WalletError) => {
        console.error(error);
    }, []);

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={[]} onError={onError} autoConnect={true}>
                <WalletModalProvider>{children}</WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
}

export function useAnchorProvider() {
    const { connection } = useConnection();
    const wallet = useWallet();

    return new AnchorProvider(connection, wallet as AnchorWallet, {
        commitment: "confirmed",
    });
}
