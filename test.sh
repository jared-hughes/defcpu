#!/bin/bash
set -euo pipefail


cargo build --release
cargo test
(
    cd tests/disassembly
    ./test-disassembly.sh
)
