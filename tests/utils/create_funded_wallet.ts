import { AnchorProvider, Provider, web3 } from '@coral-xyz/anchor';

export const createFundedWallet = async (
  provider: AnchorProvider,
  amount = 1 // SOL amount
): Promise<web3.Keypair> => {
  // create new user key pair
  const user = new web3.Keypair(); 

  // transfer SOL from provider wallet to the new user wallet
  await provider.sendAndConfirm(
    new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey, // provider wallet
        toPubkey: user.publicKey, // new user wallet
        lamports: amount * web3.LAMPORTS_PER_SOL, // amount in lamports
      })
    )
  );

  return user; // return user key pair with funds
};