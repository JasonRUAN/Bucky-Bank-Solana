"use client";

import { PublicKey, SystemProgram } from "@solana/web3.js";
import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";
import { useCluster } from "@/providers/ClusterProvider";
import { useAnchorProvider } from "@/providers/SolanaProvider";
import { BN } from "@coral-xyz/anchor";
import { useBuckyBankProgram } from "@/lib/bucky-bank-data-access";
import { Transaction } from "@solana/web3.js";
import type { BuckyBankInfo } from "@/types";

export function useCreateBuckyBank() {
    const { cluster } = useCluster();
    const provider = useAnchorProvider();
    const { program, programId, bank_global_stats_accounts } =
        useBuckyBankProgram();

    return useMutation({
        mutationKey: ["bucky-bank", "create_bucky_bank", { cluster }],
        mutationFn: async (info: BuckyBankInfo) => {
            // 获取 BankGlobalStats 账户
            const bankGlobalStats = bank_global_stats_accounts.data?.[0]?.publicKey;

            if (!bankGlobalStats) {
                throw new Error("Bank global stats not found");
            }

            // 1. 获取当前存钱罐总数
            const bankGlobalStatsAccount =
                await program.account.bankGlobalStatsInfo.fetch(bankGlobalStats);
            const buckyBankId = bankGlobalStatsAccount.totalBuckyBanks;

            console.log(`Creating bucky bank with ID: ${buckyBankId.toString()}`);

            // 2. 获取 BuckyBank PDA
            const [buckyBankPDA] = PublicKey.findProgramAddressSync(
                [buckyBankId.toArrayLike(Buffer, "le", 8)],
                programId
            );

            // 3. 获取 UserBuckyBanks PDA
            const [userBuckyBanksPDA] = PublicKey.findProgramAddressSync(
                [Buffer.from("user_bucky_banks"), provider.publicKey.toBuffer()],
                programId
            );

            console.log(`Owner: ${provider.publicKey}`);
            console.log(`Bank Global Stats: ${bankGlobalStats.toBase58()}`);
            console.log(`Bucky Bank PDA: ${buckyBankPDA.toBase58()}`);
            console.log(`User Bucky Banks PDA: ${userBuckyBanksPDA.toBase58()}`);

            const targetAmountBN = new BN(info.target_amount);
            const durationDaysBN = new BN(info.duration_days);
            const childAddress = new PublicKey(info.child_address);

            // 创建创建存钱罐的指令
            const createBuckyBankIx = await program.methods
                .createBuckyBank(
                    info.name,
                    targetAmountBN,
                    durationDaysBN,
                    childAddress
                )
                .accountsStrict({
                    owner: provider.publicKey,
                    bankGlobalStats,
                    buckyBank: buckyBankPDA,
                    userBuckyBanks: userBuckyBanksPDA,
                    systemProgram: SystemProgram.programId,
                })
                .instruction();

            // 创建交易并添加指令
            const transaction = new Transaction();
            transaction.add(createBuckyBankIx);

            // 发送并确认交易
            return await provider.sendAndConfirm(transaction);
        },
        onSuccess: (signature) => {
            console.log("Successfully created bucky bank: ", signature);
            toast.success(`创建存钱罐成功! 交易签名: ${signature}`, {
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
            toast.error(`创建存钱罐失败: ${error.message}`),
    });
}
