import { AnchorProvider, Provider } from '@coral-xyz/anchor';
import {
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddress,
} from '@solana/spl-token';
import { Keypair, PublicKey, Transaction } from '@solana/web3.js';

// finds the ATA and mint <amount> tokens 
export const createAssociatedTokenAccount = async (
  provider: AnchorProvider,
  mint: PublicKey, // mint of the ATA
  amount: number | bigint, // amount to mint
  user: Keypair // owner of the ATA
): Promise<PublicKey | undefined> => {
  // find ATA  
  const userAssociatedTokenAccount = await getAssociatedTokenAddress(
    mint, // mint
    user.publicKey // owner
  );

  // Create ATA for the user and mint some tokens
  await provider.sendAndConfirm(
    new Transaction()
      .add(
        // create ATA
        createAssociatedTokenAccountInstruction(
          user.publicKey, // payer
          userAssociatedTokenAccount, // ATA
          user.publicKey, // owner
          mint // mint
        )
      )
      .add(
        // mint tokens
        createMintToInstruction(
          mint, // mint
          userAssociatedTokenAccount, // ATA
          provider.wallet.publicKey, // mint authority
          amount // amount to mint
        )
      ),
    [user] // signer
  );

  return userAssociatedTokenAccount; // return ATA
};