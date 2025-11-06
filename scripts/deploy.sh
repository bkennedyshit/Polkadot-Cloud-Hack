#!/bin/bash

set -e

echo "🚀 ReputeChain Deployment Script"
echo "=================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check dependencies
echo -e "${YELLOW}Checking dependencies...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Install from https://rustup.rs/${NC}"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Install from https://nodejs.org/${NC}"
    exit 1
fi

if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker not found. Install from https://docker.com/${NC}"
    exit 1
fi

echo -e "${GREEN}✅ All dependencies found${NC}"

# Build backend
echo -e "${YELLOW}Building blockchain node...${NC}"
cargo build --release
echo -e "${GREEN}✅ Node built successfully${NC}"

# Build frontend
echo -e "${YELLOW}Building frontend...${NC}"
cd frontend
npm install
npm run build
cd ..
echo -e "${GREEN}✅ Frontend built successfully${NC}"

# Docker build
echo -e "${YELLOW}Building Docker images...${NC}"
docker build -t reputechain:latest .
docker build -t reputechain-frontend:latest ./frontend
echo -e "${GREEN}✅ Docker images built${NC}"

# Summary
echo ""
echo -e "${GREEN}=================================="
echo "✅ Deployment Ready!"
echo "==================================${NC}"
echo ""
echo "Next steps:"
echo "1. Run locally: make run"
echo "2. Run with Docker: make docker-up"
echo "3. Deploy to cloud: docker push reputechain:latest"
echo ""
echo "Frontend: http://localhost:3000"
echo "Node RPC: ws://localhost:9944"
echo ""
