"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { toast } from "sonner";
import { useCluster } from "@/providers/ClusterProvider";
import { useAnchorProvider } from "@/providers/SolanaProvider";
import { useBuckyBankProgram } from "@/lib/bucky-bank-data-access";

export interface WithdrawParams {
    requestId: string;
    buckyBankId: string;
}

/**
 * 执行提现 Hook
 * 调用智能合约的 withdraw 函数，执行已批准的提现申请
 */
export function useWithdraw() {
    const { cluster } = useCluster();
    const provider = useAnchorProvider();
    const { program } = useBuckyBankProgram();
    const queryClient = useQueryClient();

    return useMutation({
        mutationKey: ["bucky-bank", "withdraw", { cluster }],
        mutationFn: async ({ requestId, buckyBankId }: WithdrawParams) => {
            if (!provider.publicKey) {
                throw new Error("You need to connect your wallet first!");
            }

            console.log("Executing withdrawal...");
            console.log(`Withdrawal Request ID: ${requestId}`);
            console.log(`Bucky Bank ID: ${buckyBankId}`);
            console.log(`Child: ${provider.publicKey}`);

            const buckyBankPublicKey = new PublicKey(buckyBankId);
            const withdrawalRequestPublicKey = new PublicKey(requestId);

            // 创建提现指令
            const withdrawIx = await program.methods
                .withdraw()
                .accountsStrict({
                    buckyBank: buckyBankPublicKey,
                    withdrawalRequest: withdrawalRequestPublicKey,
                    child: provider.publicKey,
                    systemProgram: SystemProgram.programId,
                })
                .instruction();

            // 创建交易并添加指令
            const transaction = new Transaction();
            transaction.add(withdrawIx);

            // 发送并确认交易
            return await provider.sendAndConfirm(transaction);
        },
        onError: (error) => {
            console.error("Failed to withdraw: ", error);
            toast.error(
                `提现失败: ${
                    error instanceof Error ? error.message : "未知错误"
                }`
            );
        },
        onSuccess: (signature, variables) => {
            console.log("Successfully withdrew: ", signature);
            toast.success(`提现成功! 交易签名: ${signature}`, {
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

            // 刷新相关查询
            queryClient.invalidateQueries({
                queryKey: ["bucky-bank", "withdrawal_requests"],
            });
            queryClient.invalidateQueries({
                queryKey: ["bucky-bank", "bucky_banks"],
            });
        },
    });
}
