#!/bin/bash

# Exit on any error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="bloop-backend"
CONTAINER_NAME="bloop-backend-dev"
PORT="8080"

echo -e "${YELLOW}Building Docker image...${NC}"
docker build -t $IMAGE_NAME .

echo -e "${YELLOW}Stopping and removing existing container if it exists...${NC}"
docker stop $CONTAINER_NAME 2>/dev/null || true
docker rm $CONTAINER_NAME 2>/dev/null || true

# Check if pb_data directory exists and has content
if [ ! -d "pb_data" ] || [ -z "$(ls -A pb_data 2>/dev/null)" ]; then
    echo -e "${YELLOW}No existing pb_data found. Restoring from backup...${NC}"
    
    cp -r pb_data_backup pb_data
    
    echo -e "${GREEN}✅ Backup restored successfully!${NC}"
else
    echo -e "${GREEN}✅ Existing pb_data found. Skipping backup restore.${NC}"
fi

echo -e "${YELLOW}Starting new container...${NC}"
docker run -d \
--name $CONTAINER_NAME \
-p $PORT:8080 \
-v $(pwd)/pb_data:/pb/pb_data \
$IMAGE_NAME

echo -e "${GREEN}✅ Backend started successfully!${NC}"
echo -e "${GREEN}🌐 PocketBase admin UI: http://localhost:$PORT/_/admin${NC}"
echo -e "${GREEN}📡 API endpoint: http://localhost:$PORT${NC}"
echo -e "${YELLOW}📝 To view logs: docker logs -f $CONTAINER_NAME${NC}"
echo -e "${YELLOW}🛑 To stop: docker stop $CONTAINER_NAME${NC}"
