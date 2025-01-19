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

normalize() {
    # To match code.golf postprocessing, trailing spaces get trimmed,
    # and multiple trailing newlines get normalized to 1.
    sed -E 's/\s+$//g' | sed -z -E 's/\n+$/\n/'
}

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

    exp_dir="./expected/${base}"

    # mapfile splits each line into one word
    # Split at form-feeds, into the array variable 'flags'
    mapfile -d $'\f' -t flags < <(./read-initial-dump.mjs "$exp_dir/output")

    # Failure is ok here
    ../../target/release/defcpu run "$elf" "${flags[@]}" \
        1> >(normalize > "$output") \
        2> >(grep -v "Detailed error:" | normalize > "$errors") \
        || echo $'\n'"Nonzero exit code ($?) from defcpu run." >> "$errors"
        
    real_source_path="real_sources/${base}.s"
    ./insert-line-number.mjs "$errors" "$real_source_path"


    if ! cmp "$output" "$exp_dir/output" 2> /dev/null; then
        pass=0
        exit_code=1
        echo "${C_RED}fail${C_CLEAR}."
        git --no-pager diff --text --no-index "$output" "$exp_dir/output" || true
    fi
    if ! cmp "$errors" "$exp_dir/errors" 2> /dev/null; then
        pass=0
        exit_code=1
        echo "${C_RED}fail${C_CLEAR}."
        git --no-pager diff --text --no-index "$errors" "$exp_dir/errors" || true
    fi
    if [[ "$pass" != "0" ]]; then
        echo "${C_GREEN}ok${C_CLEAR}."
    fi
done
exit "$exit_code"
