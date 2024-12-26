#!/bin/bash
set -euo pipefail

(
    cd tests/decoding
    ./test-decoding.sh
)
