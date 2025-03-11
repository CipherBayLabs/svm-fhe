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
        
        await insertZero();

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
                            deposit(Number(amount), user);
                        }
                    }

                    // Look for transfer logs
                    const transferLog = logs.logs.find(log => log.includes("Transferring"));
                    const senderDepositLog = logs.logs.find(log => log.includes("Sender's deposit value:"));
                    const recipientDepositLog = logs.logs.find(log => log.includes("Recipient's deposit value:"));
                    
                    if (transferLog) {
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
                            if (recipientDepositLog) {
                                console.log('Recipient deposit value:', recipientDepositLog.split("Recipient's deposit value:")[1].trim());
                            }
                        }
                    }
                }
            },
            'confirmed'
        );

        console.log('Listening for program logs... (Press Ctrl+C to exit)');
    
        
    } catch (error) {}
};

const deposit = async (lamports: number, key: string) => {
    const value = BigInt(lamports);
    
    // Convert key string to bytes array
    const encoder = new TextEncoder();
    const keyBytes = new Uint8Array(32);
    const encodedKey = encoder.encode(key);
    
    // Copy encoded key into fixed-size array, padding with zeros if needed
    keyBytes.set(encodedKey.slice(0, 32));  // Take first 32 bytes or pad with zeros

    const requestBody = {
        value: Number(value),
        key: Array.from(keyBytes)  // Convert to regular array for JSON
    };


    console.log('Sending to Rust server:', requestBody);

    const response = await fetch('http://localhost:3000/post', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestBody)
    });
    console.log('Rust Server Response:', await response.text());
}

const transfer = async (ciphertext: string, sender: string, recipient: string) => {
    const requestBody = {
        ciphertext: ciphertext,
        sender: sender,
        recipient: recipient
    };

    const response = await fetch('http://localhost:3000/transfer', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },  
        body: JSON.stringify(requestBody)
    });
    console.log('Rust Server Response:', await response.text());
}

const insertZero = async () => {
    const requestBody = {
        key: new Array(32).fill(0),  // Creates array of 32 zeros
        value: 0
    };
    
    try {
        const response = await fetch('http://localhost:3000/post', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(requestBody)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        console.log('Successfully posted value');
    } catch (error) {
        console.error('Error posting value:', error);
    }
}

testServer(); 