export const insertZero = async () => {
    const requestBody = {
        key: new Array(32).fill(0), 
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

export const encrypt = async (value: bigint, ciphertext: any) => {
    const request = {
        key: ciphertext,
        value: Number(value)
    };
    try {
        const response = await fetch('http://localhost:3000/post', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request)
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        console.log('Successfully posted value');
    } catch (error) {
        console.error('Error posting value:', error);
    }
}

export const transfer = async (sender: any, recipient: any, transfer: any) => {
    const requestBody = {
        
    }
}