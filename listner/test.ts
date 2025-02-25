import { Connection, PublicKey } from '@solana/web3.js';

const PROGRAM_ID = new PublicKey("GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD");

const testServer = async () => {
    try {
        // Set up Solana connection (using localhost by default)
        const connection = new Connection('http://localhost:8899', {
            commitment: 'confirmed',
            wsEndpoint: 'ws://localhost:8900'
        });
        
        // Suppress WebSocket errors
        process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';
        
        // Set up program subscription
        console.log('Setting up program log listener...');
        const subscriptionId = connection.onLogs(
            PROGRAM_ID,
            (logs) => {
                if (!logs.err) {
                    // Look for deposit logs
                    const depositLog = logs.logs.find(log => log.includes("User") && log.includes("deposited"));
                    const depositInfoLog = logs.logs.find(log => log.includes("Deposit info:"));
                    if (depositLog && depositInfoLog) {
                        // Parse user and amount
                        const matches = depositLog.match(/User (.*) deposited (.*) lamports/);
                        if (matches) {
                            const user = matches[1];
                            const amount = matches[2];
                            console.log('Deposit detected:');
                            console.log('User:', user);
                            console.log('Amount:', amount);
                            console.log('Ciphertext:', depositInfoLog.split("Deposit info:")[1].trim());
                        }
                    }

                    // Look for transfer logs
                    const transferLog = logs.logs.find(log => log.includes("Transferring"));
                    const senderDepositLog = logs.logs.find(log => log.includes("Sender's deposit value:"));
                    if (transferLog) {
                        // Parse sender, receiver and ciphertext
                        const matches = transferLog.match(/Transferring (.*) from (.*) to (.*)/);
                        if (matches) {
                            const ciphertext = matches[1];
                            const sender = matches[2];
                            const receiver = matches[3];
                            console.log('Transfer detected:');
                            console.log('Sender:', sender);
                            console.log('Receiver:', receiver);
                            console.log('Transfer ciphertext:', ciphertext);
                            if (senderDepositLog) {
                                console.log('Sender deposit value:', senderDepositLog.split("Sender's deposit value:")[1].trim());
                            }
                        }
                    }
                }
            },
            'confirmed'
        );

        // Original HTTP test
        const response = await fetch('http://localhost:3000/post', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                value: 42
            })
        });
        
        const data = await response.json();
        console.log('HTTP Response:', data);

        // Keep the process running to continue listening
        console.log('Listening for program logs... (Press Ctrl+C to exit)');
        
        // Optional: To remove the subscription later
        // connection.removeOnLogsListener(subscriptionId);
        
    } catch (error) {
        // Silently catch errors
    }
};

testServer(); 