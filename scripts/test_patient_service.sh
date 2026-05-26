#!/bin/bash

# This script runs tests for the patient_service.
# It should be executed from the project root.

set -e

echo "Running tests for patient_service..."

# run all test cases in Patient service
cargo test -p patient_service -- --test-threads=1

## Run cargo test for the patient_service handlers
#cargo test -p patient_service --test handler_tests -- --test-threads=1
#
## Run cargo test for the patient_service repository
#cargo test -p patient_service --test repository_tests -- --test-threads=1

echo "All tests for patient_service passed!"
