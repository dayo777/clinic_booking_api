#!/bin/bash

# This script runs tests for the doctor_service.
# It should be executed from the project root.

set -e

echo "Running tests for doctor_service..."



# run all test on Doctor-service
cargo test -p doctor_service -- --test-threads=1

## Run cargo test for the doctor_service handlers
#cargo test -p doctor_service --test handler_tests -- --test-threads=1
#
## Run cargo test for the doctor_service repository
#cargo test -p doctor_service --test repository_tests -- --test-threads=1

echo "All tests for patient_service passed!"
