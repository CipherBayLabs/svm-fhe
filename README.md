# TFHE Encryption Server with Solana Integration

## 🔍 Overview

This project demonstrates a multi-component system for handling homomorphic encryption with blockchain verification:

1. **Rust Encryption Server**
   - Handles TFHE encryption/decryption
   - Provides REST API endpoints
   - Manages SQLite database storage
   - Generates and manages encryption keys

2. **Solana Program**
   - Records symbolic operations
   - Verifies transaction flow
   - Emits events for tracking

3. **TypeScript Listener**
   - Monitors Solana program events
   - Forwards requests to Rust server
   - Provides client interface

## 🏗 System Flow

## 🧪 Testing

The project includes multiple test suites:
- Rust unit tests for encryption logic
- Integration tests for API endpoints
- Anchor tests for Solana program
- TypeScript tests for client functionality

## 🐛 Known Issues and Solutions

### TypeScript Linter Errors

1. **Account Property Type Mismatch**
   ```typescript
   // Error: Object literal may only specify known properties
   // In blockchain/tests/blockchain.ts
   .accounts({
     depositInfo: depositInfoPDA,
     // ...
   })
   ```
   Solution: Update account types in Anchor program to match TypeScript definitions

2. **Value Type Mismatch**
   ```typescript
   // Error: Argument type '[number[]]' not assignable to parameter
   .deposit(value)
   ```
   Solution: Convert number array to BN (Big Number):
   ```typescript
   import { BN } from '@coral-xyz/anchor';
   .deposit(new BN(value))
   ```

3. **Buffer Method Error**
   ```typescript
   // Error: Property 'padEnd' does not exist on type 'Buffer'
   Buffer.from(input).slice(0, 32).padEnd(32, 0)
   ```
   Solution: Use Buffer padding method:
   ```typescript
   const buf = Buffer.from(input).slice(0, 32);
   const padded = Buffer.concat([buf, Buffer.alloc(32 - buf.length)]);
   ```
