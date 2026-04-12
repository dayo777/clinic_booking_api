#!/bin/bash

# This script runs tests for the patient_service.
# It should be executed from the project root.

set -e

echo "Running tests for patient_service..."

# Run cargo test for the patient_service package
cargo test -p patient_service --test handler_tests -- --test-threads=1

echo "All tests for patient_service passed!"
