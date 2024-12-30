#!/bin/bash
set -euo pipefail

# Script pre-req: compile to release target (cargo build --release)
# Will exit with a nonzero exit code if there's any test failure.

# Ensure `cargo dis` matches the disasembly in sources/a.s.
# The source of truth is the hexdump bytes on the left.
# The instructions on the right are the disassembly of the bytes on the right.

rm -r elfs 2> /dev/null || true
mkdir elfs
rm -r output 2> /dev/null || true
mkdir output

exit_code=0
for source_path in sources/*.s; do
    base="${source_path##sources/}"
    base="${base%.s}"

    temp_source="$(mktemp)"
    echo -n '.byte ' > "$temp_source";
    sed -E 's/(([0-9a-f]{2} )+).*/\1/g' "$source_path" \
        | tr -d '\n' \
        | sed 's/ *$//g' \
        | sed 's/ /, /g' \
        | sed -E 's/([0-9a-f]{2})/0x\1/g' \
        >> "$temp_source";
    echo >> "$temp_source";

    elf="elfs/${base}.elf"

    ../node_modules/.bin/defasm "$temp_source" -w -x -o "$elf"
    rm "$temp_source"

    output="./output/${base}.s"

    ../../target/release/defcpu dis "$elf" > "$output"
    
    longest_start=$(awk -F $"\t" '{ print length($1) }' "$output" | sort -n | tail -1)

    expand -t $(((longest_start + 4)/8*8 + 4)) "$output" | sponge "$output"

    output_dis=$(< "$output")
    exp_dis=$(< "$source_path")
    if [[ "$exp_dis" != "$output_dis" ]]; then
        exit_code=1
        echo "Test failure from '$source_path'."
        git --no-pager diff --no-index "$output" "$source_path" || true
    fi
done
exit "$exit_code"
