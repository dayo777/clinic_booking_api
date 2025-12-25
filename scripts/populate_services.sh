#!/bin/bash

# all scripts should be run from the scripts folder
# this bash scripts populates the `*_service` folders with the necessary files to get started.
# the `*_service` folder should have already been created using `cargo new my_service --lib --vcs none`

set -o errexit   # exit on command failure
set -o nounset   # treat unset variables as error

# get the name of the service to populate
read -r -p "Enter service directory name: " svc_name

# confirm the 'svc_name' input ends with '_service'
[[ "$svc_name" == *_service ]] || { echo "directory '$svc_name' does not end with '_service'." >&2; exit 1; }

# confirm present working directory is scripts
[ "$(basename "$PWD")" = "scripts" ] || { echo "Not in 'scripts' directory" >&2; exit 1; }

cd ..

# confirm the directory exists
[ $(ls -d */ | grep ${svc_name}) ] || { echo "'${svc_name}' does not exists as a workspace directory." >&2; exit 1; }
echo "'${svc_name}' directory exist..."

cd ${svc_name}

# create files here in src/
cd src
echo "// Structs: Serialize/Deserialize/Validate" > models.rs
echo "// Route handlers: get_me, update_me, etc." > handlers.rs
echo "// Database operations: insert, find_by_id, find_by_email, etc." > repository.rs
echo "// Service-specific errors" > error.rs
cd ..

# create a test directory and populate it
mkdir tests && cd tests
echo "// Integration tests against real/test DB connections." > repository_tests.rs
echo "// Unit tests for handlers" > handler_tests.rs

cd .. # back to the service directory
cd .. # back to root workspace
cd scripts # back to the scripts directory