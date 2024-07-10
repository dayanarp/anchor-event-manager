import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EventManager } from "../target/types/event_manager";
import { BN } from "bn.js";
import { Keypair, PublicKey } from '@solana/web3.js';
import { createMint, createFundedWallet, createAssociatedTokenAccount } from './utils';
import { getAssociatedTokenAddress, getAccount} from "@solana/spl-token";
import { assert } from "chai";


describe("event-manager", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EventManager as Program<EventManager>;

  // event accounts address
  let acceptedMint: PublicKey; // example: USDC

  // PDAs
  let eventPublicKey: PublicKey;
  let eventMint: PublicKey; // sponsorship token
  let treasuryVault: PublicKey;
  let gainVault: PublicKey;

  // Sponsor
  let alice: Keypair; // alice key pair
  let aliceAcceptedMintATA: PublicKey; //alice accepted mint ATA
  let aliceEventMintATA: PublicKey; //alice event mint ATA

  // provider (event organizer) wallet 
  let walletAcceptedMintATA: PublicKey; //provider wallet accepted mint ATA

  // Sponsor
  let bob: Keypair; // alice key pair
  let bobAcceptedMintATA: PublicKey; //alice accepted mint ATA
  let bobEventMintATA: PublicKey; //alice event mint ATA


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
    aliceAcceptedMintATA = await createAssociatedTokenAccount(provider,acceptedMint,500, alice);
    // find alice event mint ata (only finds address)
    aliceEventMintATA = await getAssociatedTokenAddress(eventMint, alice.publicKey);

    // find provided (event organizer) wallet acceptend mint ata
    // only the address
    walletAcceptedMintATA = await getAssociatedTokenAddress(acceptedMint, provider.wallet.publicKey);

    // create bob wallet with lamports
    bob = await createFundedWallet(provider);
    // create bob accepted mint ata
    bobAcceptedMintATA = await createAssociatedTokenAccount(provider,acceptedMint,500, bob)
    // find bob event mint ata
    bobEventMintATA = await getAssociatedTokenAddress(eventMint, bob.publicKey);

  });

  // TEST: Create an Event
  it("Creates a new Event", async () => {
    const name:string = "my_event";
    const ticketPrice = new BN(2); // 2 Accepted mint (USDC)

    const tx = await program.methods.createEvent(name, ticketPrice)
    .accounts({
      event: eventPublicKey,
      acceptedMint: acceptedMint, // example: USDC
      eventMint: eventMint, // sponsorship token
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
    console.log("Alice USDC amount before: ", aliceUSDCBalance.amount);

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
    // should have 5 sponsorship tokens
    const aliceAccount = await getAccount(
      provider.connection,
      aliceEventMintATA // Alice Event Mint account (should have <quantity> tokens from sponsorship)
    );
    console.log("Alice sponsorship tokens: ", aliceAccount.amount);

     // show alice accepted mint (USDC) ATA info
     // should have 95 (100-5) USDC
     aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice USDC amount after: ", aliceUSDCBalance.amount);
  });

  // TEST: Sponsor event
  it("Bob Should get 48 event tokens", async () => {
    const quantity = new BN(48);
    await program.methods
      .sponsorEvent(quantity)
      .accounts({
        eventMint: eventMint,
        payerAcceptedMintAta: bobAcceptedMintATA,
        event: eventPublicKey,
        authority: bob.publicKey,
        payerEventMintAta:bobEventMintATA,
        treasuryVault: treasuryVault
      })
      .signers([bob])
      .rpc();

    // show bob event mint ATA info
    const bobAccount = await getAccount(
      provider.connection,
      bobEventMintATA
    );
    console.log("Bob sponsorship tokens: ", bobAccount.amount);
  });

   // TEST: Buy Tickets
   it("Alice buy 23 tickets", async () => {
     // show alice accepted mint (USDC) ATA info
     // should have 95 USDC
     let aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice USDC amount before: ", aliceUSDCBalance.amount);

    const quantity = new BN(23); // 23 tickets
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
     console.log("Event gain vault total: ", gainVaultAccount.amount);

     // show alice accepted mint (USDC) ATA info
     // shoul have 91 (95-4) USDC
     aliceUSDCBalance = await getAccount(
      provider.connection,
      aliceAcceptedMintATA // Alice Accepted mint account (USDC account)
    );
    console.log("Alice USDC amount after: ", aliceUSDCBalance.amount);
 
   });

   // TEST: Buy 154 Tickets
   it("Bob buy 154 tickets", async () => {
    // Add your test here.
    const quantity = new BN(154);
     await program.methods
       .buyTickets(quantity)
       .accounts({
         payerAcceptedMintAta: bobAcceptedMintATA,
         event: eventPublicKey,
         authority: bob.publicKey,
         gainVault: gainVault
       })
       .signers([bob])
       .rpc();
     
     // show event gain vault info
     const gainVaultAccount = await getAccount(
       provider.connection,
       gainVault
     );
     console.log("Event gain vault amount: ", gainVaultAccount.amount);
 
   });

    // TEST: Withdraw Funds
  it("Event organizer should withdraw 1 from treasury", async () => {
   // show event treasury vault info
    // should have 5 USDC
    let treasuryVaultAccount = await getAccount(
      provider.connection,
      treasuryVault
    );
    console.log("Event treasury vault total before: ", treasuryVaultAccount.amount);

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
    treasuryVaultAccount = await getAccount(
      provider.connection,
      treasuryVault
    );
    console.log("Event treasury vault total after: ", treasuryVaultAccount.amount);

    // show event organizer accepted mint (USDC) ATA info
    // should have 1 accepted mint (USDC) 
    const organizerUSDCBalance = await getAccount(
      provider.connection,
      walletAcceptedMintATA // event organizer Accepted mint account (USDC account)
    );
    console.log("Organizer USDC amount: ", organizerUSDCBalance.amount);

  });

  // TEST: Close Event
  it("event organizer should close event", async () => {
    // act
    await program.methods
      .closeEvent()
      .accounts({
        event: eventPublicKey,
        authority: provider.wallet.publicKey
      })
      .rpc();

    // show new event info
    const eventAccount = await program.account.event.fetch(eventPublicKey);
    console.log("Event is active: ", eventAccount.active);
  });

  // TEST: Can't Buy 2 Tickets
  it("Alice can't buy tickets", async () => {
    
    let error: anchor.AnchorError;
    const quantity = new BN(2);
    try {
      await program.methods
       .buyTickets(quantity)
       .accounts({
         payerAcceptedMintAta: aliceAcceptedMintATA,
         event: eventPublicKey,
         authority: alice.publicKey,
         gainVault: gainVault
       })
       .signers([alice])
       .rpc();
    } catch (err) {
      error = err;
    }
    assert.equal(error.error.errorCode.code, "EventClosed");
    console.log("You can't buy tickets, the Event is already closed");
   });

  // TEST: Withdraw earnings
  it("Alice Should withdraw earnings", async () => {
    
    // show total sponsorships
    const eventAccount = await program.account.event.fetch(eventPublicKey);
    console.log("Event total sponsorships: ", eventAccount.sponsors.toNumber());

   // show event gain vault amount
   let gainVaultAccount = await getAccount(
     provider.connection,
     gainVault
   );
   console.log("Event gain vault amount: ", gainVaultAccount.amount);

   // show Alice sponsorship tokens
   let aliceTokens = await getAccount(
     provider.connection,
     aliceEventMintATA
   );
   console.log("Alice sponsorship tokens: ", aliceTokens.amount);
   
   await program.methods
     .withdrawEarnings()
     .accounts({
       userEventMintAta: aliceEventMintATA,
       event: eventPublicKey,
       authority: alice.publicKey,
       gainVault: gainVault,
       userAcceptedMintAta: aliceAcceptedMintATA,
       eventMint: eventMint
     })
     .signers([alice])
     .rpc();
   
   // show event gain vault amount
   gainVaultAccount = await getAccount(
     provider.connection,
     gainVault
   );
   console.log("Event gain vault amount: ", gainVaultAccount.amount);
 });

});
