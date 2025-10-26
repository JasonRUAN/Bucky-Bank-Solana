// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { Cluster, PublicKey } from "@solana/web3.js";
import BuckyBankIDL from "../target/idl/bucky_bank.json";
import type { BuckyBank } from "../target/types/bucky_bank";

// Re-export the generated IDL and type
export { BuckyBank, BuckyBankIDL };

// The programId is imported from the program IDL.
export const bucky_bank_PROGRAM_ID = new PublicKey(BuckyBankIDL.address);

// This is a helper function to get the Counter Anchor program.
export function getBuckyBankProgram(
  provider: AnchorProvider,
  address?: PublicKey
): Program<BuckyBank> {
  return new Program(
    {
      ...BuckyBankIDL,
      address: address ? address.toBase58() : BuckyBankIDL.address,
    } as BuckyBank,
    provider
  );
}

// This is a helper function to get the program ID for the Counter program depending on the cluster.
export function getBuckyBankProgramId(cluster: Cluster) {
  switch (cluster) {
    case "devnet":
    case "testnet":
      // This is the program ID for the Counter program on devnet and testnet.
      return new PublicKey("2N4emW88bMtPELSz6s62oTt48enm2yo8KbSdk9BeGPBG");
    case "mainnet-beta":
    default:
      return bucky_bank_PROGRAM_ID;
  }
}
