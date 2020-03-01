#!/bin/bash
set -e

if [ "$ENV" = 'DEV']; then
    echo "Running Development Service"
    exec cargo run
elif ["$ENV" = 'TEST']; then
    echo "Running Unit Tests"
    exec cargo test
else
    echo "Running Production Service"
    exec cargo run --release
fi
