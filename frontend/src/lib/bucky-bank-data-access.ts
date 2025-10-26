import { useQuery } from "@tanstack/react-query";
import { Cluster, PublicKey } from "@solana/web3.js";
import { useMemo } from "react";
import { useCluster } from "@/providers/ClusterProvider";
import { useAnchorProvider } from "@/providers/SolanaProvider";
import { getBuckyBankProgram, getBuckyBankProgramId } from "@project/anchor";

const BANK_GLOBAL_STATS_SEED = "global_stats";
const USER_BUCKY_BANKS_SEED = "user_bucky_banks";

export function useBuckyBankProgram() {
    const { cluster } = useCluster();
    const provider = useAnchorProvider();

    const programId = useMemo(
        () => getBuckyBankProgramId(cluster.network as Cluster),
        [cluster]
    );

    const program = useMemo(
        () => getBuckyBankProgram(provider, programId),
        [provider, programId]
    );

    const bank_global_stats_accounts = useQuery({
        queryKey: ["bucky-bank", "bank_global_stats", { cluster }],
        queryFn: async () => {
            const [bankGlobalStatsAddress] = PublicKey.findProgramAddressSync(
                [Buffer.from(BANK_GLOBAL_STATS_SEED)],
                programId
            );

            const account = await program.account.bankGlobalStatsInfo.fetch(
                bankGlobalStatsAddress
            );

            return [
                {
                    publicKey: bankGlobalStatsAddress,
                    account,
                },
            ];
        },
        enabled: !!provider,
    });

    const bucky_bank_accounts = useQuery({
        queryKey: ["bucky-bank", "bucky_banks", { cluster }],
        queryFn: async () => {
            return await program.account.buckyBankInfo.all();
        },
        enabled: !!provider,
    });

    return {
        program,
        programId,
        bank_global_stats_accounts,
        bucky_bank_accounts,
    };
}
