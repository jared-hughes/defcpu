#!/bin/bash
set -euo pipefail

./make.sh apply
cargo build --release
cargo test
(
    cd tests/disassembly
    ./test-disassembly.sh
)
(
    cd tests/integration
    ./test-run.sh
)
