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
                if (!logs.err) {  // Only log if there's no error
                    console.log('Program Log:', logs.logs);
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