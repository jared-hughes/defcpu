#!/bin/bash
set -euo pipefail

echo
echo "[Testing 'defcpu dis']"

# Script pre-req: compile to release target (cargo build --release)
# Will exit with a nonzero exit code if there's any test failure.

# Ensure `cargo dis` matches the disasembly in sources/a.s.
# The source of truth is the hexdump bytes on the left.
# The instructions on the right are the disassembly of the bytes on the right.

# show_color will give a zero (yes) exit code when connected to a terminal (tty).
show_color() { [ -t 1 ]; }

C_RED=$'\033[31m'
show_color || C_RED=''
C_GREEN=$'\033[32m'
show_color || C_GREEN=''
C_CLEAR=$'\033[0m'
show_color || C_CLEAR=''

rm -r elfs 2> /dev/null || true
mkdir elfs
rm -r output 2> /dev/null || true
mkdir output

exit_code=0
for source_path in sources/*.s; do
    base="${source_path##sources/}"
    base="${base%.s}"

    echo -n "$base..."

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
        echo "${C_RED}fail${C_CLEAR}."
        git --no-pager diff --no-index "$output" "$source_path" || true
    else
        echo "${C_GREEN}ok${C_CLEAR}."
    fi
done
exit "$exit_code"
