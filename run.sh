#!/bin/bash

# Store the root project directory
PROJECT_DIR=$(pwd)

# Kill any existing processes
echo "Killing existing processes..."
pkill -f solana-test-validator
pkill -f "cargo run"
pkill -f "ts-node"

echo "Starting Solana validator..."
cd $PROJECT_DIR/blockchain  # Change to anchor project directory
solana-test-validator --reset &
sleep 5  # Wait for validator to start

echo "Starting TypeScript listner..."
cd $PROJECT_DIR/listner && ts-node listner.ts &
sleep 5

echo "Starting Rust server..."
cd $PROJECT_DIR && cargo run &
sleep 5

echo "Running Anchor tests..."
cd $PROJECT_DIR/blockchain  # Correct path
anchor test --skip-local-validator

# Keep script running
sleep 5

echo "Cleaning up processes..."
pkill -9 -f solana-test-validator  # Force kill (-9)
pkill -9 -f "cargo run"
pkill -9 -f "ts-node"
echo "All processes terminated"