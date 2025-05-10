# ElizaOS SVM-FHE Plugin

This plugin integrates ElizaOS with the SVM-FHE (Solana Virtual Machine - Fully Homomorphic Encryption) server, enabling AI agents to perform encrypted operations on the Solana blockchain.

## Features

- Deposit values to be encrypted and stored
- Retrieve and decrypt values
- Transfer encrypted values between accounts
- Withdraw encrypted values
- Generate random account keys
- Check FHE server status
- Telegram bot integration for interacting with the FHE server

## Prerequisites

Before using this plugin, you need to have the SVM-FHE server running. Please follow the [Improved Setup Guide](../docs/improved-setup.md) to set up and run the SVM-FHE server.

## Installation

```bash
# Install the plugin in your ElizaOS project
elizaos project add-plugin @elizaos/plugin-svm-fhe
```

## Configuration

Add the plugin to your ElizaOS project configuration in `src/index.ts`:

```typescript
import { defineCharacter } from '@elizaos/core';
import SvmFhePlugin from '@elizaos/plugin-svm-fhe';

export default defineCharacter({
  name: 'My Character',
  plugins: [
    new SvmFhePlugin({
      fheServerUrl: 'http://localhost:3000',
      solanaRpcUrl: 'http://localhost:8899',
      programId: 'GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD'
    })
  ],
  // ... other character configuration
});
```

## Usage

Once the plugin is installed and configured, your ElizaOS agent can use the following actions:

### Deposit a Value

```typescript
const result = await agent.actions.deposit([1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 100);
console.log(result); // { success: true, status: 200 }
```

### Decrypt a Value

```typescript
const result = await agent.actions.decrypt([1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
console.log(result); // { success: true, value: 100 }
```

### Transfer Between Accounts

```typescript
const result = await agent.actions.transfer(
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], // sender
  [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], // recipient
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]  // transfer amount (encrypted 0)
);
console.log(result); // { success: true, status: 200 }
```

### Withdraw

```typescript
const result = await agent.actions.withdraw(
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], // account
  [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]  // withdrawal amount (encrypted 0)
);
console.log(result); // { success: true, newBalance: 100 }
```

### Generate a Random Key

```typescript
const result = await agent.actions.generateKey();
console.log(result); // { key: [123, 0, 0, 0, ...] }
```

### Check Server Status

```typescript
const result = await agent.actions.checkServerStatus();
console.log(result); // { status: 'online' }
```

## Telegram Bot Integration

The plugin includes a Telegram bot integration that allows users to interact with the FHE server through Telegram messages. This provides a user-friendly interface for managing encrypted assets.

### Configuration

To enable the Telegram bot integration, you need to provide a bot token in your ElizaOS project configuration:

```typescript
import { defineCharacter } from '@elizaos/core';
import SvmFhePlugin from '@elizaos/plugin-svm-fhe';

export default defineCharacter({
  name: 'My Character',
  plugins: [
    new SvmFhePlugin({
      fheServerUrl: 'http://localhost:3000',
      solanaRpcUrl: 'http://localhost:8899',
      programId: 'GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD',
      telegram: {
        botToken: 'YOUR_TELEGRAM_BOT_TOKEN',
        allowDirectMessages: true,
        shouldOnlyJoinInAllowedGroups: false,
        allowedGroupIds: []
      }
    })
  ],
  // ... other character configuration
});
```

### Telegram Bot Commands

The Telegram bot supports the following commands:

- `/start` - Welcome message and list of available commands
- `/help` - Show help message with available commands
- `/deposit <value>` - Deposit a value to be encrypted
- `/balance` - Check your encrypted balance
- `/transfer <recipient_id> <amount>` - Transfer encrypted funds
- `/status` - Check if the FHE server is running

### Starting the Bot Programmatically

You can also start the Telegram bot programmatically using the `startTelegramBot` action:

```typescript
const result = await agent.actions.startTelegramBot('YOUR_TELEGRAM_BOT_TOKEN', {
  allowDirectMessages: true,
  shouldOnlyJoinInAllowedGroups: false,
  allowedGroupIds: []
});
console.log(result); // { success: true }
```

### Stopping the Bot

To stop the Telegram bot:

```typescript
const result = await agent.actions.stopTelegramBot();
console.log(result); // { success: true }
```

## Example Conversation Flow

Here's an example of how an AI agent might use this plugin in a conversation:

```
User: I want to deposit 50 tokens into my encrypted account.

Agent: I'll help you deposit 50 tokens into an encrypted account. Let me generate a new account key for you.

[Agent uses generateKey action]

Agent: I've generated a new account key for you. Now I'll deposit 50 tokens into this account.

[Agent uses deposit action with the generated key and value 50]

Agent: Great! I've successfully deposited 50 tokens into your encrypted account. Would you like to check the balance?

User: Yes, please check the balance.

[Agent uses decrypt action with the same key]

Agent: Your current balance is 50 tokens. This value is stored in encrypted form on the blockchain and can only be accessed with the proper authorization.
```

## Troubleshooting

If you encounter issues with the plugin, check the following:

1. Ensure the SVM-FHE server is running on the specified URL (default: http://localhost:3000)
2. Verify that the Solana test validator is running (if using Solana integration)
3. Check that the keys used in requests are properly formatted as 32-byte arrays

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT
