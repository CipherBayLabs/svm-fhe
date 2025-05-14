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

        const [storagePDA_A] = await PublicKey.findProgramAddress(
            [Buffer.from("fhe_storage"), Buffer.from(a)],
            fheLibId
        );
            
        const [storagePDA_B] = await PublicKey.findProgramAddress(
            [Buffer.from("fhe_storage"), Buffer.from(b)],
            fheLibId
        );

        const [storagePDA_Sum] = await PublicKey.findProgramAddress(
            [Buffer.from("fhe_storage")],
            appProgram.programId  // Note: Using app program ID here, not fheLibId
        );

        console.log("storagePDA_A", storagePDA_A.toBase58());
        console.log("storagePDA_B", storagePDA_B.toBase58());
        console.log("storagePDA_Sum", storagePDA_Sum.toBase58());

        // Set up event listener
        // let eventFound = false;
        // const listener = appProgram.addEventListener("Add8", (event, slot) => {
        //     console.log("Add8 event:", event);
        //     // Verify event data
        //     expect(event.lhs).to.deep.equal(a);
        //     expect(event.rhs).to.deep.equal(b);
        //     expect(event.sum).to.not.be.null;
        //     eventFound = true;
        // });
             
        // await appProgram.removeEventListener(listener);


        // Call the app program's test_first_add method
        await appProgram.methods
        .testFirstAdd(a, b)
        .accounts({
            signer: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
            fheLib: fheLibId,
            storageSum: storagePDA_Sum,
            storageA: storagePDA_A,
            storageB: storagePDA_B,
            
        })
        .rpc();

        // Verify both storage accounts were created
        const storageAccount_A = await provider.connection.getAccountInfo(storagePDA_A);
        const storageAccount_B = await provider.connection.getAccountInfo(storagePDA_B);
        
        expect(storageAccount_A).to.not.be.null;
        expect(storageAccount_B).to.not.be.null;


    })
})