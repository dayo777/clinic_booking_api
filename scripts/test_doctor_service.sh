#!/bin/bash

# This script runs tests for the doctor_service.
# Reason for deleting containers is, testing was creating too many containers on my local machine

set +e
echo "Running tests for doctor_service..."

# run all test on Doctor-service
cargo test -p doctor_service -- --test-threads=1
TEST_EXIT_CODE=$?

# Delete all containers created by testcontainers
echo "Cleaning up containers..."
docker ps -a --filter "label=org.testcontainers.managed-by=testcontainers" -q | xargs -r docker rm -f

if [ $TEST_EXIT_CODE -ne 0 ]; then
    echo "Tests failed with exit code $TEST_EXIT_CODE"
    exit $TEST_EXIT_CODE
fi

echo "All tests for doctor_service passed!"
