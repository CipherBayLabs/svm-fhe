import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blockchain } from "../target/types/blockchain";
import { PublicKey } from '@solana/web3.js';
import { assert } from 'chai';

describe("blockchain", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Blockchain as Program<Blockchain>;
  const newUser = anchor.web3.Keypair.generate();
  const provider = anchor.getProvider();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can deposit SOL", async () => {
    const amount = new anchor.BN(1_000_000_000);
    
    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    // Get PDA for deposit info
    const [depositInfoPDA] = PublicKey.findProgramAddressSync(
      [provider.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .deposit(amount)
      .accounts({
        depositInfo: depositInfoPDA,
        vault: vaultPDA,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit transaction signature", tx);
  });

  it("Can deposit SOL from another account", async () => {
    // Create new account with some SOL
    
    const signature = await provider.connection.requestAirdrop(
        newUser.publicKey,
        2_000_000_000  // 2 SOL (extra for fees)
    );
    await provider.connection.confirmTransaction(signature);

    // Get PDAs
    const [vaultPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault")],
        program.programId
    );

    const [depositInfoPDA] = PublicKey.findProgramAddressSync(
        [newUser.publicKey.toBuffer()],  // Use new user's pubkey
        program.programId
    );

    // Deposit 1 SOL
    const amount = new anchor.BN(1_000_000_000);
    const tx = await program.methods
        .deposit(amount)
        .accounts({
            depositInfo: depositInfoPDA,
            vault: vaultPDA,
            user: newUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([newUser])  // Add new user as signer
        .rpc();

    console.log("Second deposit transaction signature", tx);
});


  it("Can transfer SOL", async () => {
    // Use helper to generate random value
    const value = generateRandomBytes32();

    // Derive PDA using the value as seeds
    const [depositInfoPDA] = PublicKey.findProgramAddressSync(
      [provider.publicKey.toBuffer()],
      program.programId
    );

    // Generate new recipient
    const recipient = newUser.publicKey;

    // Then transfer (just emits events)
    const tx = await program.methods
        .transfer(value, recipient)
        .accounts({
            depositInfo: depositInfoPDA,
            user: provider.publicKey,
            recipient: recipient,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    console.log("Transfer transaction signature", tx);
    console.log("Random value used:", value);
    console.log("Recipient:", recipient.toString());
  });

});







/////////////////// helper functions ///////////////////


function toBytes32(input: number[] | string | Buffer): number[] {
  if (Array.isArray(input)) {
      // If array, pad or truncate to 32 bytes
      return Array(32).fill(0).map((_, i) => input[i] || 0);
  } 
  if (typeof input === 'string') {
      // If hex string, remove 0x and convert
      const hex = input.startsWith('0x') ? input.slice(2) : input;
      const bytes = Buffer.from(hex.padStart(64, '0'), 'hex');
      return Array.from(bytes);
  }
  // If buffer, convert to array
  return Array.from(Buffer.from(input).slice(0, 32).padEnd(32, 0));
}

function generateRandomBytes32(): number[] {
  return toBytes32(Array(32).fill(0).map(() => Math.floor(Math.random() * 256)));
}