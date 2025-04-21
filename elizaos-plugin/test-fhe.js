const axios = require('axios');

// FHE Server URL
const FHE_SERVER_URL = 'http://localhost:3000';

// Configure axios with a timeout
const api = axios.create({
  baseURL: FHE_SERVER_URL,
  timeout: 5000 // 5 second timeout
});

// Test keys
const zeroKey = Array(32).fill(0);
const userKey = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const recipientKey = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const transferValueKey = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// FHE Actions
async function checkServerStatus() {
  try {
    const response = await api.post('/decrypt', { key: zeroKey });
    console.log('✅ FHE Server is online. Zero key value:', response.data.result);
    return true;
  } catch (error) {
    console.error('❌ FHE Server is offline or not responding:', error.message);
    return false;
  }
}

async function deposit(key, value) {
  try {
    const response = await api.post('/post', { key, value });
    console.log(`✅ Deposited ${value} tokens to account with key:`, key[0]);
    return true;
  } catch (error) {
    console.error('❌ Deposit failed:', error.message);
    return false;
  }
}

async function decrypt(key) {
  try {
    const response = await api.post('/decrypt', { key });
    console.log(`✅ Account balance for key ${key[0]}: ${response.data.result} tokens`);
    return response.data.result;
  } catch (error) {
    console.error('❌ Decrypt failed:', error.message);
    return null;
  }
}

async function transfer(senderKey, recipientKey, transferValueKey) {
  try {
    const response = await api.post('/transfer', {
      sender_key: senderKey,
      recipient_key: recipientKey,
      transfer_value: transferValueKey
    });
    console.log(`✅ Transferred tokens from account ${senderKey[0]} to account ${recipientKey[0]}`);
    return true;
  } catch (error) {
    console.error('❌ Transfer failed:', error.message);
    return false;
  }
}

// Run tests
async function runTests() {
  console.log('🔍 Testing SVM-FHE Integration');
  console.log('============================');
  
  // Check server status
  const serverOnline = await checkServerStatus();
  if (!serverOnline) {
    console.error('❌ Cannot proceed with tests - FHE server is offline');
    return;
  }
  
  console.log('\n📥 Testing deposit functionality');
  // Deposit 100 tokens to user account
  await deposit(userKey, 100);
  
  console.log('\n🔢 Testing decrypt functionality');
  // Check user balance
  const userBalance = await decrypt(userKey);
  
  // Prepare for transfer test
  console.log('\n📥 Depositing transfer amount to temporary account');
  await deposit(transferValueKey, 50);
  
  console.log('\n↔️ Testing transfer functionality');
  // Transfer 50 tokens from user to recipient
  await transfer(userKey, recipientKey, transferValueKey);
  
  console.log('\n🔢 Checking balances after transfer');
  // Check balances after transfer
  await decrypt(userKey);
  await decrypt(recipientKey);
  
  console.log('\n✅ All tests completed');
}

// Run the tests
runTests().catch(error => {
  console.error('❌ Test failed with error:', error);
});
