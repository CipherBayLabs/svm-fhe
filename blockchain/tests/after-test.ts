import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blockchain } from "../target/types/blockchain";
import { PublicKey } from '@solana/web3.js';

describe("Secondary Test Suite", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Blockchain as Program<Blockchain>;

  it("Retrieves stored [u8; 32] value", async () => {
    console.log("Checking balance for pubkey:", provider.publicKey.toString());
    
    const [depositInfoPDA] = PublicKey.findProgramAddressSync(
      [provider.publicKey.toBuffer()],
      program.programId
    );
    console.log("Deposit Info PDA:", depositInfoPDA.toString());

    const accountInfo = await program.account.depositInfo.fetch(depositInfoPDA);
    console.log("Stored value ([u8; 32]):", accountInfo.value);
    console.log("Value length:", accountInfo.value.length);  // Should be 32
    console.log("Owner of this deposit:", accountInfo.owner.toString());  // Should match provider.publicKey
  });
});