import { clusterApiUrl, PublicKey } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey(
  "GMRpg29rcyvoYS5XnzXy8mC1qV58xRB5zWkSVLBsuhc3",
);

export const RPC_URL =
  process.env.RPC_URL ||
  process.env.NEXT_PUBLIC_RPC_URL ||
  clusterApiUrl("mainnet-beta");
