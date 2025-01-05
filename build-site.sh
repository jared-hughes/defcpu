#!/bin/bash
set -euo pipefail

build_wasm() {
    PACK_DIR="$PWD/node_modules/defcpu_web"
    (
        cd defcpu-web
        wasm-pack build --target web --out-dir "${PACK_DIR}"
    )
    cp "${PACK_DIR}"/defcpu_web.d.ts site/src/
}

build_static() {
    if [ -d public-deploy ]; then
        rm -rf public-deploy;
    fi

    mkdir -p public-deploy/js
    (
        cd site
        node esbuild.mjs
    )
    cp "${PACK_DIR}"/*.{wasm,js} public-deploy/js
    rsync -a site/public/ public-deploy/
}

build_wasm
build_static
