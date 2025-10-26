"use client";

import { ReactQueryProvider } from "@/providers/ReactQueryProvider";
import { SolanaProvider } from "@/providers/SolanaProvider";
import { ClusterProvider } from "@/providers/ClusterProvider";

export default function SolanaDappProvider({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <ReactQueryProvider>
            <ClusterProvider>
                <SolanaProvider>{children}</SolanaProvider>
            </ClusterProvider>
        </ReactQueryProvider>
    );
}
