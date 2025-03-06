# TFHE Encryption Server with Solana Integration

The idea here is to create a off-chain server that handles FHE operations symbolically on the SVM. Essentially when an on-chain action is needed, an event will be emited and orginazed via a TS server where it then will be forwarded to the Rust backend where the encryption, FHE opertaions and decryption requests are handled. Next steps include on-chain verification. 

## 🔍 Overview

This project demonstrates a multi-component system for handling homomorphic encryption with blockchain verification:

1. **Rust Encryption Server**
   - Handles TFHE encryption/decryption
   - Provides REST API endpoints
   - Manages SQLite database storage
   - Generates and manages encryption keys (seperate script)

2. **Solana Program**
   - Records symbolic operations
   - Verifies transaction flow
   - Emits events for tracking

3. **TypeScript Listener**
   - Monitors Solana program events
   - Forwards requests to Rust server
   - Provides client interface

## 🧪 Testing

The project includes multiple test suites:
- Rust unit tests for encryption logic
- Integration tests for API endpoints
- Anchor tests for Solana program
- TypeScript tests for client functionality

## 🐛 Known Issues and Solutions





