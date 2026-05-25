#!/bin/bash

# This script deletes other docker containers created during testing
# remember to enable script using `chmod +x ./scripts/delete_other_docker_containers.sh`
set -e

# Use the first argument as the container name to keep, or default to 'jaeger'
CONTAINER_TO_KEEP=${1:-"jaeger"}

echo "Deleting other Containers except the one named ${CONTAINER_TO_KEEP}"

# Find the container ID of the container to keep
EXCLUDE_ID=$(docker ps -aq -f name=^/"${CONTAINER_TO_KEEP}"$ || true)

if [ -n "$EXCLUDE_ID" ]; then
    # Delete all containers except the one to keep
    docker ps -aq | grep -v "$EXCLUDE_ID" | xargs -r docker rm -f
else
    # If no such container exists, delete all containers
    docker ps -aq | xargs -r docker rm -f
fi

echo "All docker containers deleted except ${CONTAINER_TO_KEEP}!"
