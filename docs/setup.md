# Running a Local FHE Coprocessor

This is a guide to running you own local FHE coprocessor. You can either use this local for testing or you can deploy this to any SVM netwrok in order to ad FHE directly to your programs with a managed Client and Server Key. If you are interested in distributing the decryption key via MPC please check out https://github.com/zama-ai/threshold-fhe

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

1. Install Solana:
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

2. Install Anchor:
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install 0.30.1
   avm use 0.30.1
   ```

## Setup

Step 1: Clone the repo: 
   - `git clone https://github.com/kkoshiya/svm-fhe.git`
   - `cd svm-fhe`

Step 2: We now need to generate the encryption keys. This is done by running the following command: `cargo run --bin generate_keys` or `sh generate.sh`

    This Should Generate a "keys" folder in the root of the project with the following files:
    - `client_key.bin`
    - `server_key.bin`

Step 3: Open a new terminal end enter the blockchain directory: `cd blockchain`

   Enter the Following Commands:
   - `anchor build`
   - `anchor keys sync`
   - `anchor build`

Step 4: Open a new terminal end enter the frontend directory: `cd listner`

   Enter the Following Commands:
   - `npm install`

   Then update the programId to the same one in the blockchain Program.

Step 5: After this intial setup we can start running each component in their own terminal.

   Run each in separate terminals:

   1. `cargo run`                      # Root directory: FHE Server
   2. `solana-test-validator --reset`  # Any directory: Validator
   3. `ts-node relayer.ts`            # In listener: Relayer
   4. `npm run test:primary`          # In blockchain: Tests



