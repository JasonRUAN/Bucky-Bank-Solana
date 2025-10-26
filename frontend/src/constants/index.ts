export enum QueryKey {
    GetFileQueryKey = "GetFileQueryKey",
    GetBuckyBanks = "GetBuckyBanks",
    GetBuckyBankById = "GetBuckyBankById",
    GetDepositsByBuckyBankId = "GetDepositsByBuckyBankId",
    GetWithdrawsByBuckyBankId = "GetWithdrawsByBuckyBankId",
    GetWithdrawalRequestsByBuckyBankId = "GetWithdrawalRequestsByBuckyBankId",
    GetWithdrawalRequestsByRequester = "GetWithdrawalRequestsByRequester",
    GetParentPendingRequests = "GetParentPendingRequests",
}

export const CONSTANTS = {
    API: {
        BASE_URL: process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001",
    },
};
