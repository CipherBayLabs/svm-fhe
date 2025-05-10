import axios from 'axios';

/**
 * FHE Actions for ElizaOS
 * 
 * These actions enable ElizaOS to interact with the Solana FHE server for
 * performing encrypted operations on the Solana blockchain.
 */

interface FheConfig {
  serverUrl: string;
}

/**
 * Deposit a value to be encrypted
 * @param key - 32-byte array representing the account key
 * @param value - The value to encrypt and store
 * @param config - Configuration options
 */
export async function deposit(key: number[], value: number, config: FheConfig) {
  try {
    const response = await axios.post(`${config.serverUrl}/post`, {
      key,
      value
    });
    return { success: true, status: response.status };
  } catch (error) {
    console.error('FHE deposit error:', error);
    return { success: false, error: (error as Error).message };
  }
}

/**
 * Decrypt and retrieve a value
 * @param key - 32-byte array representing the account key
 * @param config - Configuration options
 */
export async function decrypt(key: number[], config: FheConfig) {
  try {
    const response = await axios.post(`${config.serverUrl}/decrypt`, { key });
    return { success: true, value: response.data.result };
  } catch (error) {
    console.error('FHE decrypt error:', error);
    return { success: false, error: (error as Error).message };
  }
}

/**
 * Transfer encrypted value between accounts
 * @param senderKey - 32-byte array representing the sender's account key
 * @param recipientKey - 32-byte array representing the recipient's account key
 * @param transferValueKey - 32-byte array representing the transfer amount key
 * @param config - Configuration options
 */
export async function transfer(
  senderKey: number[], 
  recipientKey: number[], 
  transferValueKey: number[], 
  config: FheConfig
) {
  try {
    const response = await axios.post(`${config.serverUrl}/transfer`, {
      sender_key: senderKey,
      recipient_key: recipientKey,
      transfer_value: transferValueKey
    });
    return { success: true, status: response.status };
  } catch (error) {
    console.error('FHE transfer error:', error);
    return { success: false, error: (error as Error).message };
  }
}

/**
 * Withdraw encrypted value
 * @param key - 32-byte array representing the account key
 * @param valueKey - 32-byte array representing the withdrawal amount key
 * @param config - Configuration options
 */
export async function withdraw(key: number[], valueKey: number[], config: FheConfig) {
  try {
    const response = await axios.post(`${config.serverUrl}/withdraw`, {
      key,
      value: valueKey
    });
    return { success: true, newBalance: response.data.result };
  } catch (error) {
    console.error('FHE withdraw error:', error);
    return { success: false, error: (error as Error).message };
  }
}

/**
 * Check if the FHE server is running
 * @param config - Configuration options
 */
export async function checkServerStatus(config: FheConfig) {
  try {
    // Try to access the zero key as a basic connectivity test
    const response = await axios.post(`${config.serverUrl}/decrypt`, { 
      key: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] 
    });
    return { status: 'online', value: response.data.result };
  } catch (error) {
    return { status: 'offline', error: (error as Error).message };
  }
}

/**
 * Generate a random 32-byte key array
 */
export function generateKey() {
  const key = new Array(32).fill(0).map((_, i) => i === 0 ? Math.floor(Math.random() * 255) + 1 : 0);
  return { key };
}

/**
 * Generate a deterministic 32-byte key from a user ID
 * @param userId - User ID to generate key from
 */
export function generateKeyFromUserId(userId: string) {
  const key = new Array(32).fill(0);
  
  // Convert userId to a byte array
  for (let i = 0; i < Math.min(userId.length, 32); i++) {
    key[i] = parseInt(userId[i]) || 1;
  }
  
  // Ensure first byte is non-zero
  if (key[0] === 0) {
    key[0] = 1;
  }
  
  return { key };
}
