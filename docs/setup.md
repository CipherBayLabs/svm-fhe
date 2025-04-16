# Running a Local FHE Coprocessor

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

Step 1: Clone the repo: `git clone https://github.com/kkoshiya/svm-fhe.git`

Step 2: `cd svm-fhe`

Step 3: We now need to generate the encryption keys. This is done by running the following command: `cargo run --bin generate_keys` or `./ generate.sh`

    This Should Generate a "keys" folder in the root of the project with the following files:
    - `client_key.bin`
    - `server_key.bin`

Step 4:  