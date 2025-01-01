#!/bin/bash
set -euo pipefail

if [ -d public-deploy ]; then
    rm -rf public-deploy;
fi

PACK_DIR="$PWD/node_modules/defcpu_web"
(
    cd defcpu-web
    wasm-pack build --target web --out-dir "${PACK_DIR}"
)
mkdir -p public-deploy/js
cp "${PACK_DIR}"/defcpu_web.d.ts site/src/
(
    cd site
    node esbuild.mjs
)
cp "${PACK_DIR}"/*.{wasm,js} public-deploy/js
rsync -a site/public/ public-deploy/
