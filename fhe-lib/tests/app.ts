import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { App } from "../target/types/app";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("app", () => {

    anchor.setProvider(anchor.AnchorProvider.env());
    const provider = anchor.getProvider() as anchor.AnchorProvider;
    const appProgram = anchor.workspace.App as Program<App>
    const fheLibId = new anchor.web3.PublicKey("Fuj5qpvT66C7pz4fvyLDV6d8YCUS9idJH2i66Qj5vedh");

    it ("adds two ciphertexts", async () => {
        const a = Array.from(anchor.web3.Keypair.generate().publicKey.toBytes().slice(0, 32));
        const b = Array.from(anchor.web3.Keypair.generate().publicKey.toBytes().slice(0, 32));

        const [storagePDA] = await PublicKey.findProgramAddress(
            [Buffer.from("fhe_storage"), Buffer.from(a)],
            fheLibId
        );
        
        console.log("Using storage PDA:", storagePDA.toString());
        
        await appProgram.methods
        .testFirstAdd(a, b)
        .accounts({
          signer: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          fheLib: fheLibId,
          storage: storagePDA,
        })
        .rpc();

        const storageAccount = await provider.connection.getAccountInfo(storagePDA);
        expect(storageAccount).to.not.be.null;
        expect(storageAccount.owner.toString()).to.equal(fheLibId.toString());

    })
})