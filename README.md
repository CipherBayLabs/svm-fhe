# TFHE Encryption Server with Solana Integration

The idea here is to create a off-chain server that handles FHE operations symbolically on the SVM. Essentially when an on-chain action is needed, an event will be emited and orginazed via a TS server where it then will be forwarded to the Rust backend where the encryption, FHE opertaions and decryption requests are handled. The Goal is to build a generalized FHE coprocessor that is native to the SVM, ideally including a wide range of operations, different FHE schemes and threshold decryption. Next steps include on-chain verification. 

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

## Architecture 

The process first begins when a user deposits a certain amount of lamports into the program via the deposit function. This will create a mapping from the user's address to a ciphertext that represents their lamport value. (down the road this can also be used for SPL tokens such as USDC). 
![naughty.drawio.pdf](https://github.com/user-attachments/files/19120696/naughty.drawio.pdf)




