"use client";

import { useMutation } from "@tanstack/react-query";
import { LAMPORTS_PER_SOL, SystemProgram, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import { toast } from "sonner";
import { useCluster } from "@/providers/ClusterProvider";
import { useAnchorProvider } from "@/providers/SolanaProvider";
import { useBuckyBankProgram } from "@/lib/bucky-bank-data-access";

export interface DepositParams {
    buckyBankId: string;
    amount: number;
}

/**
 * 存款 Hook
 * 调用智能合约的 deposit 函数
 */
export function useDeposit() {
    const { cluster } = useCluster();
    const provider = useAnchorProvider();
    const { program, bank_global_stats_accounts } =
        useBuckyBankProgram();

    return useMutation({
        mutationKey: ["bucky-bank", "deposit", { cluster }],
        mutationFn: async ({ buckyBankId, amount }: DepositParams) => {
            if (!provider.publicKey) {
                throw new Error("You need to connect your wallet first!");
            }

            // 获取 BankGlobalStats 账户
            const bankGlobalStats = bank_global_stats_accounts.data?.[0]?.publicKey;

            if (!bankGlobalStats) {
                throw new Error("Bank global stats not found");
            }

            // 获取 BuckyBank PDA
            const buckyBankPDA =  buckyBankId;

            console.log(`Depositing to bucky bank with ID: ${buckyBankId.toString()}`);
            console.log(`Bank Global Stats: ${bankGlobalStats.toBase58()}`);
            console.log(`Bucky Bank PDA: ${buckyBankPDA}`);
            console.log(`Depositor: ${provider.publicKey}`);

            const depositAmountBN = new BN(amount);

            // 创建存款指令
            const depositIx = await program.methods
                .deposit(depositAmountBN)
                .accountsStrict({
                    bankGlobalStats,
                    buckyBank: buckyBankPDA,
                    depositor: provider.publicKey,
                    systemProgram: SystemProgram.programId,
                })
                .instruction();

            // 创建交易并添加指令
            const transaction = new Transaction();
            transaction.add(depositIx);

            // 发送并确认交易
            return await provider.sendAndConfirm(transaction);
        },
        onSuccess: (signature) => {
            console.log("Successfully deposited: ", signature);
            toast.success(`存款成功! 交易签名: ${signature}`, {
                action: {
                    label: "查看",
                    onClick: () => {
                        window.open(
                            `https://explorer.solana.com/tx/${signature}?cluster=devnet`,
                            "_blank"
                        );
                    },
                },
            });
        },
        onError: (error) =>
            toast.error(`存款失败: ${error.message}`),
    });
}


/**
 * 将 SOL 转换为 Lamports (1 SOL = 1,000,000,000 Lamports)
 */
export function solToLamports(solAmount: string): string {
    const sol = parseFloat(solAmount);
    if (isNaN(sol)) {
        throw new Error("无效的SOL金额");
    }
    return (sol * LAMPORTS_PER_SOL).toString();
}

/**
 * 将 Lamports 转换为 SOL
 */
export function lamportsToSol(lamportsAmount: string): string {
    const lamports = parseInt(lamportsAmount);
    if (isNaN(lamports)) {
        throw new Error("无效的Lamports金额");
    }
    return (lamports / LAMPORTS_PER_SOL).toString();
}

export function amountToSOL(amount: string): string {
    const SOL = parseFloat(amount);
    if (isNaN(SOL)) {
        throw new Error("无效的SOL金额");
    }
    return (SOL * LAMPORTS_PER_SOL).toString();
}

export function SOLToAmount(SOLAmount: string): string {
    const amount = parseInt(SOLAmount);
    if (isNaN(amount)) {
        throw new Error("无效的SOL金额");
    }
    return (amount / LAMPORTS_PER_SOL).toString();
}
