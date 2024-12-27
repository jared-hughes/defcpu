#!/bin/bash
set -euo pipefail


cargo build --release
(
    cd tests/disassembly
    ./test-disassembly.sh
)
