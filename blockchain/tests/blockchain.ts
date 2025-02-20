import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blockchain } from "../target/types/blockchain";
import { PublicKey } from '@solana/web3.js';

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
});
