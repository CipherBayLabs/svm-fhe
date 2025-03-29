import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blockchain } from "../target/types/blockchain";
import { PublicKey } from '@solana/web3.js';

describe("Secondary Test Suite", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Blockchain as Program<Blockchain>;

  it("Retrieves stored [u8; 32] value and decrypts", async () => {
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

    // Call decrypt endpoint using same format as working endpoints
    try {
        console.log('Sending decrypt request...');
        const response = await fetch('http://localhost:3000/decrypt', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ key: valueArray })
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`HTTP error! status: ${response.status}, body: ${errorText}`);
        }

        const data = await response.json();
        console.log("Decrypted value:", data.result);
    } catch (error) {
        console.error('Detailed decrypt error:', error);
        throw error;
    }
    
    console.log("Owner of this deposit:", accountInfo.owner.toString());
  });



  
});


