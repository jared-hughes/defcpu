#!/bin/bash
set -euo pipefail

# A "make" script-ish.
(
    cd tests/integration
    ./run-sources.sh
    ./build-elfs.sh
)
(
    cd tests/disassembly
    ./validate-sources.sh "$@"
)
