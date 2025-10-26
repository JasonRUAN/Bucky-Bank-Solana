"use client";

import { useMutation } from "@tanstack/react-query";
import { PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import { toast } from "sonner";
import { useCluster } from "@/providers/ClusterProvider";
import { useAnchorProvider } from "@/providers/SolanaProvider";
import { useBuckyBankProgram } from "@/lib/bucky-bank-data-access";

export interface RequestWithdrawalParams {
    buckyBankId: string;
    amount: number;
    reason: string;
}

/**
 * 申请提现 Hook
 * 调用智能合约的 request_withdrawal 函数
 */
export function useRequestWithdrawal() {
    const { cluster } = useCluster();
    const provider = useAnchorProvider();
    const { program, programId } = useBuckyBankProgram();

    return useMutation({
        mutationKey: ["bucky-bank", "request_withdrawal", { cluster }],
        mutationFn: async ({ buckyBankId, amount, reason }: RequestWithdrawalParams) => {
            if (!provider.publicKey) {
                throw new Error("You need to connect your wallet first!");
            }

            console.log("Submitting withdrawal request...");
            console.log(`Bucky Bank PDA: ${buckyBankId}`);
            console.log(`Amount: ${amount}`);
            console.log(`Reason: ${reason}`);
            console.log(`Requester: ${provider.publicKey}`);

            // 获取存钱罐状态以获取 withdrawal_request_counter
            const buckyBankPublicKey = new PublicKey(buckyBankId);
            const buckyBankAccount = await program.account.buckyBankInfo.fetch(buckyBankPublicKey);
            
            // 生成 withdrawal_request PDA（包含计数器）
            const counterBuffer = Buffer.alloc(8);
            counterBuffer.writeBigUInt64LE(BigInt(buckyBankAccount.withdrawalRequestCounter.toString()), 0);
            
            const [withdrawalRequestPDA] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("withdrawal_request"),
                    buckyBankPublicKey.toBuffer(),
                    provider.publicKey.toBuffer(),
                    counterBuffer,
                ],
                programId
            );

            console.log(`Withdrawal Request PDA: ${withdrawalRequestPDA.toBase58()}`);

            const withdrawalAmountBN = new BN(amount);

            // 创建取款请求指令
            const requestWithdrawalIx = await program.methods
                .requestWithdrawal(withdrawalAmountBN, reason)
                .accountsStrict({
                    buckyBank: buckyBankId,
                    withdrawalRequest: withdrawalRequestPDA,
                    requester: provider.publicKey,
                    systemProgram: SystemProgram.programId,
                })
                .instruction();

            // 创建交易并添加指令
            const transaction = new Transaction();
            transaction.add(requestWithdrawalIx);

            // 发送并确认交易
            return await provider.sendAndConfirm(transaction);
        },
        onError: (error) => {
            console.error("Failed to submit withdrawal request: ", error);
            toast.error(`提现申请失败: ${error.message}`);
        },
        onSuccess: (signature) => {
            console.log("Successfully submitted withdrawal request: ", signature);
            toast.success(`提现申请成功! 交易签名: ${signature}`, {
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
    });
}
