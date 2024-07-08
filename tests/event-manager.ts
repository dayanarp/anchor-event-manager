import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EventManager } from "../target/types/event_manager";
import { BN } from "bn.js";
import { Keypair, PublicKey } from '@solana/web3.js';
import { createMint, createFundedWallet, createAssociatedTokenAccount } from './utils';
import { getAssociatedTokenAddress, getAccount} from "@solana/spl-token";


describe("event-manager", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EventManager as Program<EventManager>;

  // event accounts address
  let acceptedMint: PublicKey; // example: USDC

  // PDAs
  let eventPublicKey: PublicKey;
  let eventMint: PublicKey;
  let treasuryVault: PublicKey;
  let gainVault: PublicKey;

  // Sponsor
  let alice: Keypair; // alice key pair
  let aliceAcceptedMintATA: PublicKey; //alice accepted mint ATA
  let aliceEventMintATA: PublicKey; //alice event mint ATA

  // provider (event organizer) wallet 
  let walletAcceptedMintATA: PublicKey; //provider wallet accepted mint ATA


  // all this should exists **before** calling our program instructions
  before(async () => {
    acceptedMint = await createMint(provider);

    // find event account PDA
    [eventPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event", "utf-8"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // find event mint account PDA
    [eventMint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event_mint", "utf-8"), eventPublicKey.toBuffer()],
      program.programId
    );

    // find treasury vault account PDA
    [treasuryVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury_vault", "utf-8"), eventPublicKey.toBuffer()],
      program.programId
    );

    // find gain vault account PDA
    [gainVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("gain_vault", "utf-8"), eventPublicKey.toBuffer()],
      program.programId
    );

    // creates a new wallet funded with 3 SOL 
    alice = await createFundedWallet(provider, 3);
    // create alice accepted mint ata with 100 accepted mint
    // Accepted mint = USDC  -> alice wallet = 100 USDC 
    aliceAcceptedMintATA = await createAssociatedTokenAccount(provider,acceptedMint,100, alice);
    // find alice event mint ata (only finds address)
    aliceEventMintATA = await getAssociatedTokenAddress(eventMint, alice.publicKey);

    // find provided (event organizer) wallet acceptend mint ata
    // only the address
    walletAcceptedMintATA = await getAssociatedTokenAddress(acceptedMint, provider.wallet.publicKey);

  });

  // TEST: Create an Event
  it("Creates a new Event", async () => {
    const name:string = "my_event";
    const ticketPrice = new BN(2); // 2 Accepted mint (USDC)

    const tx = await program.methods.createEvent(name, ticketPrice)
    .accounts({
      event: eventPublicKey,
      acceptedMint: acceptedMint, // example: USDC
      eventMint: eventMint,
      treasuryVault: treasuryVault,
      gainVault: gainVault,
      authority: provider.wallet.publicKey, // event organizer
    })
    .rpc();

     // show new event info
     const eventAccount = await program.account.event.fetch(eventPublicKey);
     console.log("Event info: ", eventAccount);
  });

   // TEST: Sponsor event
   it("Alice Should get 5 event tokens", async () => {
     // show alice accepted mint (USDC) ATA info
     // should have 100 USDC
     let aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice Accepted mint ATA: ", aliceUSDCBalance);

    const quantity = new BN(5); // 5 USDC 
    await program.methods
      .sponsorEvent(quantity)
      .accounts({
        eventMint: eventMint, // 1:1 with USDC
        payerAcceptedMintAta: aliceAcceptedMintATA, // Alice USDC Account 
        event: eventPublicKey,
        authority: alice.publicKey,
        payerEventMintAta:aliceEventMintATA, // Alice Event Mint Account
        treasuryVault: treasuryVault // store all Accepted mint (USDC) from sponsorships
      })
      .signers([alice])
      .rpc();

    // show alice event mint ATA info
    // should have 5 Event mint
    const aliceAccount = await getAccount(
      provider.connection,
      aliceEventMintATA // Alice Event Mint account (should have <quantity> tokens from sponsorship)
    );
    console.log("Alice Event mint ATA: ", aliceAccount);

     // show alice accepted mint (USDC) ATA info
     // should have 95 (100-5) USDC
     aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice Accepted mint ATA: ", aliceUSDCBalance);
  });

   // TEST: Buy 2 Tickets
   it("Alice buy 2 tickets", async () => {
     // show alice accepted mint (USDC) ATA info
     // should have 95 USDC
     let aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice Accepted mint ATA: ", aliceUSDCBalance);

    const quantity = new BN(2); // 2 tickets
     await program.methods
       .buyTickets(quantity) 
       .accounts({
         payerAcceptedMintAta: aliceAcceptedMintATA, // Alice Accepted mint (USDC) account
         event: eventPublicKey,
         authority: alice.publicKey,
         gainVault: gainVault // stores all accepted mint (USDC) from tickets purchase
       })
       .signers([alice])
       .rpc();
     
     // show event gain vault info
     // should have 4 USDC ( 2 tickets x 2 USDC (tickect_price))
     const gainVaultAccount = await getAccount(
       provider.connection,
       gainVault // stores all accepted mint (USDC) from tickets purchase
     );
     console.log("Event gain vault: ", gainVaultAccount);

     // show alice accepted mint (USDC) ATA info
     // shoul have 91 (95-4) USDC
     aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice Accepted mint ATA: ", aliceUSDCBalance);
 
   });

    // TEST: Withdraw Funds
  it("Event organizer should withdraw funds", async () => {
   
    const amount = new BN(1); // 1 USDC
    await program.methods
      .withdrawFunds(amount)
      .accounts({
        event: eventPublicKey,
        acceptedMint: acceptedMint, // example: USDC
        authority: provider.wallet.publicKey, // event organizer
        treasuryVault: treasuryVault, // stores all Accepted Mint (USDC) from sponsorships
        authotiryAcceptedMintAta: walletAcceptedMintATA, // account where the event organizer receives accepted mint(USDC)
      })
      .rpc();
    
    // show event treasury vault info
    // should have 4 (5-1) USDC
    const treasuryVaultAccount = await getAccount(
      provider.connection,
      treasuryVault
    );
    console.log("Event treasury vault: ", treasuryVaultAccount);

    // show event organizer accepted mint (USDC) ATA info
    // should have 1 accepte mint (USDC) 
    const organizerUSDCBalance = await getAccount(
      provider.connection,
      walletAcceptedMintATA // event organizer Accepted mint account (USDC account)
    );
    console.log("Alice Accepted mint ATA: ", organizerUSDCBalance);

  });

});
