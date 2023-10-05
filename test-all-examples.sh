#!/bin/bash

# TODO: optimize script and log output

EXAMPLES_DIR="./examples"

COMMAND="cargo test --features e2e-tests"

find "$EXAMPLES_DIR" -name Cargo.toml | while read cargo_file; do
    dir=$(dirname "$cargo_file")
    echo "Testing crate in directory: $dir"
    (cd "$dir" && cargo clean && $COMMAND)
    if [ $? -ne 0 ]; then
        echo "Tests failed in $dir"
        exit 1
    fi
done
