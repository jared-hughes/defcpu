#!/bin/bash
set -euo pipefail

rm -r elfs 2> /dev/null || true
mkdir elfs

# Build sources/*.s into elfs/*.elf with defasm
for source_path in sources/*.s; do
    base="${source_path##sources/}"
    base="${base%.s}"
    real_asm_path="real_sources/$base.s"
    ./insert-macros.mjs "$source_path" "$real_asm_path"
    ../../node_modules/.bin/defasm "$real_asm_path" -w -x -o "elfs/${base}.elf"
done
