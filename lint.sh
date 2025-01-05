#!/bin/bash
set -euo pipefail

cargo clippy
./node_modules/.bin/tsc --build
