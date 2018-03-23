#!/usr/bin/env bash

set -e

if [[ -n "$CLIPPY" ]]; then
    cargo clippy -- -Dclippy
else
    cargo test
fi
