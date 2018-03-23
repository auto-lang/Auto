#!/usr/bin/env bash

set -e

if [[ -n "$CLIPPY" ]]; then
    if ! cargo install clippy --debug --force; then
        echo "COULD NOT COMPILE CLIPPY, IGNORING CLIPPY TESTS"
        exit
    fi
    cargo clippy -- -Dclippy
else
    cargo test
fi
