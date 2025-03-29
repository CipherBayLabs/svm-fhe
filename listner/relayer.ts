import { Connection, PublicKey } from '@solana/web3.js';
import { insertZero, encrypt, transfer } from './fhe';

const PROGRAM_ID = new PublicKey("GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD");

async function startListener() {
    // Create connection
    const connection = new Connection('http://localhost:8899', {
        wsEndpoint: 'ws://localhost:8900',
        commitment: 'processed'
    });
    await insertZero();
    console.log('Starting listener...');

    // Subscribe to logs
    connection.onLogs(
        PROGRAM_ID,
        async (logInfo) => {
            // Check if there are any logs
            if (!logInfo.err && logInfo.logs.length > 0) {
                // Look for deposit instruction
                const depositLog = logInfo.logs.find(log => 
                    log.includes("User") && log.includes("deposited")
                );
                const depositInfoLog = logInfo.logs.find(log => 
                    log.includes("Deposit info:")
                );

                // Look for transfer logs
                const senderLog = logInfo.logs.find(log => 
                    log.includes("Sender's deposit value:")
                );
                const recipientLog = logInfo.logs.find(log => 
                    log.includes("Recipient's deposit value:")
                );
                const transferLog = logInfo.logs.find(log => 
                    log.includes("Transferring")
                );

                if (depositLog && depositInfoLog) {
                    console.log('=== Deposit Detected ===');
                    console.log('Deposit Log:', depositLog);
                    const valueStr = depositLog.split("deposited ")[1].split(" ")[0];
                    const value = BigInt(valueStr);
                    const arrayStr = depositInfoLog.split("Deposit info:")[1].trim();
                    const ciphertext = JSON.parse(arrayStr);
                    await encrypt(value, ciphertext);
                    console.log('Deposit Amount:', value);
                    console.log('Deposit Info Array:', ciphertext);
                }

                if (senderLog && recipientLog && transferLog) {
                    console.log('=== Transfer Detected ===');
                    
                    // Extract sender ciphertext (handling debug format)
                    const senderArray = senderLog.split("value: ")[1].trim()
                        .replace(/[\[\]]/g, '');  // Remove brackets
                    const senderCiphertext = JSON.parse(`[${senderArray}]`);
                    
                    // Extract recipient ciphertext
                    const recipientArray = recipientLog.split("value: ")[1].trim()
                        .replace(/[\[\]]/g, '');
                    const recipientCiphertext = JSON.parse(`[${recipientArray}]`);
                    
                    // Extract transfer amount ciphertext
                    const transferArray = transferLog.split("Transferring ")[1].split(" from")[0].trim()
                        .replace(/[\[\]]/g, '');
                    const transferCiphertext = JSON.parse(`[${transferArray}]`);

                    await transfer(senderCiphertext, recipientCiphertext, transferCiphertext);
                    
                    console.log('Sender Ciphertext:', senderCiphertext);
                    console.log('Recipient Ciphertext:', recipientCiphertext);
                    console.log('Transfer Amount Ciphertext:', transferCiphertext);
                }
            }
        },
        'confirmed'
    );

    console.log('Listening for program logs... (Press Ctrl+C to exit)');
}

startListener().catch(console.error);

