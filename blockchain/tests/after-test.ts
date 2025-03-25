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
    
    // Convert to proper byte array and format
    const valueArray = Array.from(accountInfo.value);
    console.log("Stored value ([u8; 32]):", 
      valueArray.map(b => b.toString()).join(', ')
    );
    
    // Also show hex format
    console.log("Value (hex):", 
      Buffer.from(valueArray).toString('hex')
    );
    
    console.log("Value length:", valueArray.length);
    console.log("Owner of this deposit:", accountInfo.owner.toString());
  });
});