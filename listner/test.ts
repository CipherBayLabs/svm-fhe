import { Connection, PublicKey } from '@solana/web3.js';

const PROGRAM_ID = new PublicKey("GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD");

const testServer = async () => {
    try {
        // Set up Solana connection (using localhost by default)
        const connection = new Connection('http://localhost:8899', 'confirmed');
        
        // Set up program subscription
        console.log('Setting up program log listener...');
        const subscriptionId = connection.onLogs(
            PROGRAM_ID,
            (logs) => {
                console.log('Program Log:', logs.logs);
                if (logs.err) {
                    console.error('Program Error:', logs.err);
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
        console.error('Error:', error);
    }
};

testServer(); 