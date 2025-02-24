import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blockchain } from "../target/types/blockchain";
import { PublicKey } from '@solana/web3.js';
import { assert } from 'chai';

describe("blockchain", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Blockchain as Program<Blockchain>;
  const provider = anchor.getProvider();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can deposit SOL", async () => {
    // Amount to deposit (1 SOL = 1_000_000_000 lamports)
    const amount = new anchor.BN(1_000_000_000);
    
    // Get PDA for vault
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


  it("Can transfer SOL", async () => {
    // Use helper to generate random value
    const value = generateRandomBytes32();
    
    // Derive PDA using the value as seeds
    const [depositInfoPDA] = PublicKey.findProgramAddressSync(
      [provider.publicKey.toBuffer()],
      program.programId
    );
    // Generate new recipient
    const recipient = anchor.web3.Keypair.generate();

    // First deposit to initialize the account
    await program.methods
        .deposit(value)
        .accounts({
            depositInfo: depositInfoPDA,
            user: provider.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    // Then transfer (just emits events)
    const tx = await program.methods
        .transfer(value, recipient.publicKey)
        .accounts({
            depositInfo: depositInfoPDA,
            user: provider.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    console.log("Transfer transaction signature", tx);
    console.log("Random value used:", value);
  });

  it("Can emit bytes", async () => {
    // Generate random 32 bytes
    const value = generateRandomBytes32();
    console.log("Random bytes generated:", value);
    
    // Call emit_bytes with the random value
    const tx = await program.methods
        .emitBytes(value)
        .accounts({})
        .rpc();

    console.log("Transaction signature:", tx);
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