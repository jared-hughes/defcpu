#!/bin/bash
set -euo pipefail

echo "Testing 'defcpu run'"

# Script pre-req: build-elfs.sh built into `elfs/`.
# Script pre-req: run-sources.sh ran and put results into `output/`
# Will exit with a nonzero exit code if there's any test failure.

rm -r output 2> /dev/null || true
mkdir output

exit_code=0
for source_path in sources/*.s; do
    base="${source_path##sources/}"
    base="${base%.s}"

    elf="elfs/${base}.elf"

    out_dir="./output/${base}"
    mkdir "$out_dir"
    output="$out_dir/output"
    errors="$out_dir/errors"

    ../../target/release/defcpu run "$elf" > "$output" 2> "$errors"

    exp_dir="./expected/${base}"

    output_output=$(< "$output")
    exp_output=$(< "$exp_dir/output")
    if [[ "$exp_output" != "$output_output" ]]; then
        exit_code=1
        echo "Test failure from '$source_path' output."
        git --no-pager diff --no-index "$output" "$exp_dir/output" || true
    fi
    output_errors=$(< "$errors")
    exp_errors=$(< "$exp_dir/errors")
    if [[ "$exp_errors" != "$output_errors" ]]; then
        exit_code=1
        echo "Test failure from '$source_path' errors."
        git --no-pager diff --no-index "$errors" "$exp_dir/errors" || true
    fi
    exit
    
done
exit "$exit_code"
