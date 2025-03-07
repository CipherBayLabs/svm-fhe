#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo "Building and running keys.rs..."

# Run the keys binary with the new name
if cargo run --bin generate_keys; then
    echo -e "${GREEN}Keys program completed successfully${NC}"
else
    echo -e "${RED}Keys program failed${NC}"
    exit 1
fi