#!/bin/bash
set -euo pipefail

# A "make" script-ish.
(
    cd tests/integration
    ./build-elfs.sh
    ./run-sources.sh
)
(
    cd tests/decoding
    ./validate-sources.sh "$@"
)
