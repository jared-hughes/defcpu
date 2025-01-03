#!/bin/bash
set -euo pipefail

# Ensure sources/a.s represent correct disassembly.
# The source of truth is the hexdump bytes on the left.
# The instructions on the right are the disassembly of the bytes on the right.

rm -r elfs 2> /dev/null || true
mkdir elfs
rm -r expected 2> /dev/null || true
mkdir expected

for source_path in sources/*.s; do
    base="${source_path##sources/}"
    base="${base%.s}"

    temp_source="$(mktemp)"
    echo -n '.byte ' > "$temp_source";
    sed -E 's/([0-9a-fA-F]{2}( [0-9a-fA-F]{2})*)( |$).*/\1/g' "$source_path" \
        | tr '\n' ' ' \
        | sed -E 's/ +/ /g' \
        | sed -E 's/ /, /g' \
        | sed -E 's/([0-9a-fA-F]{2})/0x\1/g' \
        | sed -E 's/, $//g' \
        >> "$temp_source";

    echo >> "$temp_source";

    elf="elfs/${base}.elf"

    ../../node_modules/.bin/defasm "$temp_source" -w -x -o "$elf"
    
    rm "$temp_source"

    # start_addr looks like 0x0000000000400000
    start_addr=$(objdump -x "$elf" | grep 'start address' | awk '{ print $3 }')
    # segment_len looks like 0x0000000000000049
    segment_len=$(objdump -x "$elf" | awk 'c{print $4;c=0}; /vaddr '"$start_addr"'/{c=1};');

    expected="./expected/${base}.s"

    gdb -e "$elf" -batch -ex "b *$start_addr" -ex 'r' \
        -ex "disassemble /r $start_addr, +$segment_len" \
        | tail -n +5 \
        | head -n -1 \
        | sed -E 's/(=>)? *0x[0-9a-fA-F]+:\s*//g' \
        | sed -E 's/,/, /g' \
        | sed -E 's/,  /, /g' \
        | sed -E 's/ *$//g' \
        | sed -E 's/ *#.*//g' \
        > "$expected"
    
    longest_start=$(awk -F $"\t" '{ print length($1) }' "$expected" | sort -n | tail -1)

    expand -t $(((longest_start + 4)/8*8 + 4)) "$expected" | sponge "$expected"

    exp_dis=$(< "$expected")
    old_dis=$(< "$source_path")
    if [[ "$old_dis" != "$exp_dis" ]]; then
        echo "Difference in '$source_path'."
        git --no-pager diff --no-index "$source_path" "$expected" || true
        if [[ "${1:-}" == "apply" ]]; then
            echo "Applying new contents for '$source_path'...";
            echo "If the file is open in your editor, you may wish to revert it to the version on disk.";
            cp "$expected" "$source_path"
        else
            # shellcheck disable=SC2016
            echo 'Run as `'"$0"' apply` (or via `./make.sh apply`) to modify the file in-place.'
        fi
    fi
done
