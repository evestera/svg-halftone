#!/usr/bin/env bash

set -ex

cargo build --target "$TARGET" --verbose
cargo test --target "$TARGET" --verbose
