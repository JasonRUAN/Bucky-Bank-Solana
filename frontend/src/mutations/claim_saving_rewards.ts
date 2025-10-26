import { useMutation } from "@tanstack/react-query";
import { useWallet } from "@solana/wallet-adapter-react";
import { useConnection } from "@solana/wallet-adapter-react";
import { PublicKey, Transaction } from "@solana/web3.js";
import { toast } from "sonner";

export interface ClaimParams {
    buckyBankId: string;
}

export function useClaimSavingRewards({ buckyBankId }: ClaimParams) {
    const { publicKey, sendTransaction } = useWallet();
    const { connection } = useConnection();

    return useMutation({
        mutationFn: async () => {
            if (!publicKey) {
                throw new Error("You need to connect your wallet first!");
            }

            const tx = new Transaction();

            console.log("Claiming saving rewards...");
            // 这里需要根据你的 Solana 合约来调整
            // 示例：调用合约的 claim_reward 函数

            // 发送交易
            const signature = await sendTransaction(tx, connection);

            // 等待确认
            await connection.confirmTransaction(signature, "confirmed");

            return signature;
        },
        onError: (error) => {
            console.error("Failed to claim rewards: ", error);
            toast.error(`领取奖励失败: ${error.message}`);
        },
        onSuccess: (signature) => {
            console.log("Successfully claimed rewards: ", signature);
            toast.success(`领取奖励成功! 交易签名: ${signature}`, {
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
