#!/bin/bash
set -euo pipefail

echo
echo "[Testing 'defcpu run']"

# Script pre-req: build-elfs.sh built into `elfs/`.
# Script pre-req: run-sources.sh ran and put results into `output/`
# Will exit with a nonzero exit code if there's any test failure.

# show_color will give a zero (yes) exit code when connected to a terminal (tty).
show_color() { [ -t 1 ]; }

C_RED=$'\033[31m'
show_color || C_RED=''
C_GREEN=$'\033[32m'
show_color || C_GREEN=''
C_CLEAR=$'\033[0m'
show_color || C_CLEAR=''

rm -r output 2> /dev/null || true
mkdir output

exit_code=0
for source_path in sources/*.s; do
    base="${source_path##sources/}"
    base="${base%.s}"
    pass=1
    echo -n "$base..."

    elf="elfs/${base}.elf"

    out_dir="./output/${base}"
    mkdir "$out_dir"
    output="$out_dir/output"
    errors="$out_dir/errors"

    # Failure is ok here
    ../../target/release/defcpu run "$elf" > "$output" 2> "$errors" || \
        echo $'\n'"Nonzero exit code ($?) from defcpu run." >> "$errors"

    ./insert-line-number.mjs "$errors" "$source_path"

    exp_dir="./expected/${base}"

    output_output=$(< "$output")
    exp_output=$(< "$exp_dir/output")
    if [[ "$exp_output" != "$output_output" ]]; then
        pass=0
        exit_code=1
        echo "${C_RED}fail${C_CLEAR}."
        git --no-pager diff --no-index "$output" "$exp_dir/output" || true
    fi
    output_errors=$(< "$errors")
    exp_errors=$(< "$exp_dir/errors")
    if [[ "$exp_errors" != "$output_errors" ]]; then
        pass=0
        exit_code=1
        echo "${C_RED}fail${C_CLEAR}."
        git --no-pager diff --no-index "$errors" "$exp_dir/errors" || true
    fi
    if [[ "$pass" != "0" ]]; then
        echo "${C_GREEN}ok${C_CLEAR}."
    fi
done
exit "$exit_code"
