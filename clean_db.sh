#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Checking for data directory...${NC}"

if [ -d "data" ]; then
    echo -e "${YELLOW}Found data directory. Deleting...${NC}"
    rm -rf data
    echo -e "${GREEN}Data directory successfully deleted${NC}"
else
    echo -e "${RED}No data directory found${NC}"
fi