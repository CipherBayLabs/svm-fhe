#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' 

echo -e "${YELLOW}Checking for keys directory...${NC}"

if [ -d "keys" ]; then
    echo -e "${YELLOW}Found keys directory. Deleting...${NC}"
    rm -rf keys
    echo -e "${GREEN}Keys directory successfully deleted${NC}"
else
    echo -e "${RED}No keys directory found${NC}"
fi