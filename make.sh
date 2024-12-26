#!/bin/bash
set -euo pipefail

# A "make" script-ish.
(
    cd tests
    ./build-elfs.sh
    ./run-sources.sh
)
