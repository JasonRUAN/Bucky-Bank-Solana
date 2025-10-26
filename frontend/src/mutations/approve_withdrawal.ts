"use client";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { PublicKey, Transaction } from "@solana/web3.js";
import { toast } from "sonner";
import { useCluster } from "@/providers/ClusterProvider";
import { useAnchorProvider } from "@/providers/SolanaProvider";
import { useBuckyBankProgram } from "@/lib/bucky-bank-data-access";

export interface ApproveWithdrawalParams {
    requestId: string;
    buckyBankId: string;
    approve: boolean;
    reason: string;
}

/**
 * 审批提现申请 Hook
 * 调用智能合约的 approve_withdrawal 函数
 */
export function useApproveWithdrawal() {
    const { cluster } = useCluster();
    const provider = useAnchorProvider();
    const { program } = useBuckyBankProgram();
    const queryClient = useQueryClient();

    return useMutation({
        mutationKey: ["bucky-bank", "approve_withdrawal", { cluster }],
        mutationFn: async ({
            requestId,
            buckyBankId,
            approve,
            reason,
        }: ApproveWithdrawalParams) => {
            if (!provider.publicKey) {
                throw new Error("You need to connect your wallet first!");
            }

            // 验证参数
            const errors = validateApproveWithdrawalParams({
                requestId,
                buckyBankId,
                approve,
                reason,
            });
            if (errors.length > 0) {
                throw new Error(errors.join(", "));
            }

            const action = approve ? "Approving" : "Rejecting";
            console.log(`${action} withdrawal request...`);
            console.log(`Withdrawal RequestID PDA: ${requestId}`);
            console.log(`Bucky Bank ID: ${buckyBankId}`);
            console.log(`Reason: ${reason}`);
            console.log(`Parent: ${provider.publicKey}`);

            const buckyBankPublicKey = new PublicKey(buckyBankId);
            const withdrawalRequestPublicKey = new PublicKey(requestId);

            // 创建审批指令
            const approveWithdrawalIx = await program.methods
                .approveWithdrawal(approve, reason)
                .accountsStrict({
                    buckyBank: buckyBankPublicKey,
                    withdrawalRequest: withdrawalRequestPublicKey,
                    parent: provider.publicKey,
                })
                .instruction();

            // 创建交易并添加指令
            const transaction = new Transaction();
            transaction.add(approveWithdrawalIx);

            // 发送并确认交易
            return await provider.sendAndConfirm(transaction);
        },
        onError: (error) => {
            console.error("Failed to process withdrawal approval: ", error);
            toast.error(
                `审批失败: ${
                    error instanceof Error ? error.message : "未知错误"
                }`
            );
        },
        onSuccess: (signature, variables) => {
            console.log(
                "Successfully processed withdrawal approval: ",
                signature
            );
            const action = variables.approve ? "批准" : "拒绝";
            toast.success(
                `提现申请${action}成功! 交易签名: ${signature}`,
                {
                    action: {
                        label: "查看",
                        onClick: () => {
                            window.open(
                                `https://explorer.solana.com/tx/${signature}?cluster=devnet`,
                                "_blank"
                            );
                        },
                    },
                }
            );

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

/**
 * 验证审批提现参数
 */
export function validateApproveWithdrawalParams(
    params: Partial<ApproveWithdrawalParams>
): string[] {
    const errors: string[] = [];

    if (!params.requestId) {
        errors.push("提现申请ID不能为空");
    }

    if (!params.buckyBankId) {
        errors.push("存钱罐ID不能为空");
    }

    if (params.approve === undefined || params.approve === null) {
        errors.push("审批结果不能为空");
    }

    if (!params.reason) {
        errors.push("审批理由不能为空");
    } else if (params.reason.trim().length === 0) {
        errors.push("审批理由不能为空");
    }

    return errors;
}
