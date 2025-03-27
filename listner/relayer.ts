import { Connection, PublicKey } from '@solana/web3.js';

const PROGRAM_ID = new PublicKey("GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD");

async function startListener() {
    // Create connection
    const connection = new Connection('http://localhost:8899', {
        wsEndpoint: 'ws://localhost:8900',
        commitment: 'confirmed'
    });

    console.log('Starting listener...');

    // Subscribe to logs
    connection.onLogs(
        PROGRAM_ID,
        (logInfo) => {
            // Check if there are any logs
            if (!logInfo.err && logInfo.logs.length > 0) {
                // Look for deposit instruction
                const depositLog = logInfo.logs.find(log => 
                    log.includes("User") && log.includes("deposited")
                );
                const depositInfoLog = logInfo.logs.find(log => 
                    log.includes("Deposit info:")
                );

                if (depositLog && depositInfoLog) {
                    console.log('=== Deposit Detected ===');
                    console.log('Deposit Log:', depositLog);
                    const arrayStr = depositInfoLog.split("Deposit info:")[1].trim();
                    console.log('Deposit Info Array:', arrayStr);
                    const ciphertext = JSON.parse(arrayStr);
                }
            }
        },
        'confirmed'
    );

    console.log('Listening for program logs... (Press Ctrl+C to exit)');
}

startListener().catch(console.error);

