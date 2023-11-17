#!/bin/bash

# TODO: optimize script and log output

EXAMPLES_DIR="./examples"

# Command template
COMMAND="cargo test"

find "$EXAMPLES_DIR" -name Cargo.toml | while read cargo_file; do
    dir=$(dirname "$cargo_file")
    echo "Testing crate in directory: $dir"

    # Check if the 'e2e-tests' feature is defined in Cargo.toml
    if grep -q 'e2e-tests' "$cargo_file"; then
        echo "'e2e-tests' feature found, running tests with the feature"
        full_command="$COMMAND --features e2e-tests"
    else
        echo "'e2e-tests' feature not found, running tests without the feature"
        full_command="$COMMAND"
    fi

    # Run the test command
    (cd "$dir" && cargo clean && $full_command -- --test-threads=4 && cargo clean)
    if [ $? -ne 0 ]; then
        echo "Tests failed in $dir"
        exit 1
    fi
done
