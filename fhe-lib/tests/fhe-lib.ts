import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FheLib } from "../target/types/fhe_lib";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("fhe_lib", () => {

  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.FheLib as Program<FheLib>
  
  xit ("Creates a ciphertext", async () => {
    const key = Array.from(anchor.web3.Keypair.generate().publicKey.toBytes().slice(0, 32));
    
    const [storagePDA] = await PublicKey.findProgramAddress(
      [Buffer.from("fhe_storage"), Buffer.from(key)],
      program.programId
    );

    // @ts-ignore - The type system doesn't match the actual accounts structure
    await program.methods.asFhe8(key).accounts({
      storage: storagePDA,
      signer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    const storageAccount = await program.account.cipherText.fetch(storagePDA);
    
    // Simple verification
    expect(storageAccount.bitLength).to.equal(8);
    
    console.log("Storage account created successfully!");
    console.log("Bit length:", storageAccount.bitLength);
    console.log("Owner:", storageAccount.owner.toString());

  })

});
