# Improved Setup Guide for SVM-FHE

This guide provides detailed instructions for setting up and running the SVM-FHE project, addressing common issues and providing troubleshooting tips.

## Prerequisites

Required versions:
```bash
node >= 16.0.0
npm >= 7.0.0
anchor-cli >= 0.30.1
solana-cli >= 1.17.0
rustc >= 1.70.0
```

## Installation

1. **Install Rust (if not already installed or if version is outdated)**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   rustup default stable
   rustup update
   ```

2. **Install Solana**:
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

3. **Install Anchor**:
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install 0.30.1
   avm use 0.30.1
   ```

## Setup

### Step 1: Clone the repository
```bash
git clone https://github.com/kkoshiya/svm-fhe.git
cd svm-fhe
```

### Step 2: Generate encryption keys
```bash
cargo run --bin generate_keys
```

This should generate a "keys" folder in the root of the project with the following files:
- `client_key.bin`
- `server_key.bin`

If you encounter any issues with dependencies, try running:
```bash
cargo update
```

### Step 3: Running the components

For the best experience, run each component in a separate terminal:

#### Terminal 1: Start the FHE Server
```bash
cd /path/to/svm-fhe
cargo run
```

The server should start on http://localhost:3000

#### Terminal 2: Start the Solana Test Validator
```bash
cd /path/to/svm-fhe
solana-test-validator --reset
```

#### Terminal 3: Start the TypeScript Listener/Relayer
```bash
cd /path/to/svm-fhe/listner
npm install
ts-node relayer.ts
```

## Verifying the Setup

You can verify that the system is working correctly by making HTTP requests to the FHE server:

1. **Check if the zero key is initialized**:
   ```bash
   curl -X POST -H "Content-Type: application/json" -d '{"key": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}' http://localhost:3000/decrypt
   ```
   Expected response: `{"result":0}`

2. **Deposit a value**:
   ```bash
   curl -X POST -H "Content-Type: application/json" -d '{"key": [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], "value": 100}' http://localhost:3000/post
   ```

3. **Retrieve the deposited value**:
   ```bash
   curl -X POST -H "Content-Type: application/json" -d '{"key": [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}' http://localhost:3000/decrypt
   ```
   Expected response: `{"result":100}`

## Troubleshooting

### Issue: Cargo.lock version incompatibility
If you encounter an error like:
```
error: failed to parse lock file at: /path/to/svm-fhe/blockchain/Cargo.lock
Caused by: lock file version `4` was found, but this version of Cargo does not understand this lock file, perhaps Cargo needs to be updated?
```

**Solution**: Make sure you're using the latest stable version of Rust and Cargo:
```bash
rustup update stable
rustup default stable
```

### Issue: Database errors
If you encounter database-related errors, try cleaning the database:
```bash
rm -f data/tfhe.db
```

### Issue: TypeScript listener/relayer errors
If you encounter errors with the TypeScript listener/relayer, make sure you have the correct dependencies installed:
```bash
cd /path/to/svm-fhe/listner
npm install
```

## Understanding the System Architecture

The SVM-FHE system consists of three main components:

1. **FHE Server**: A Rust server that handles Fully Homomorphic Encryption operations. It provides REST API endpoints for encryption, decryption, and FHE operations.

2. **Solana Program**: A Solana program that records symbolic operations and emits events.

3. **TypeScript Listener/Relayer**: A component that monitors events from the Solana program and forwards requests to the FHE server.

The flow of operations is as follows:
1. The Solana program records symbolic operations and emits events
2. The listener/relayer picks up these events and forwards them to the FHE server
3. The FHE server performs the actual encryption, homomorphic operations, and decryption
