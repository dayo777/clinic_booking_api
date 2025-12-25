#!/bin/sh

# all scripts should be run from the scripts folder

echo "Running Rustfmt across all workspaces..."
cargo fmt --all

echo "Running Clippy across all workspaces..."
cargo clippy --workspace -- -D warnings
