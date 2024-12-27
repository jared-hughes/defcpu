#!/bin/bash
set -euo pipefail


cargo build --release
(
    cd tests/decoding
    ./test-decoding.sh
)
