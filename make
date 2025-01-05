#!/bin/bash
set -euo pipefail
# https://stackoverflow.com/a/24112741
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

pushd "$parent_path" > /dev/null
./scripts/esbuild-scripts.mjs
node target/ts-scripts/make.js "$@"
popd > /dev/null
