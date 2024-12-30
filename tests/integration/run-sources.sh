#!/bin/bash
set -euo pipefail

mkdir -p expected

# TODO remove old expected

# Turn sources/* into expected/* by running on code.golf servers
for source_file in sources/*.s; do
    base="${source_file##sources/}"
    base="${base%.s}"
    out_dir="./expected/${base}"

    # Check if it's already ran
    sha="$(sha256sum "$source_file")"
    old_sha="$(cat "./expected/${base}/sha256sum" 2>/dev/null || true)"
    if [[ "$old_sha" == "$sha" ]]; then
        continue
    fi

    # Stale or missing output
    echo "Updating ${base}."
    rm -r "${out_dir}" 2> /dev/null || true
    mkdir -p "${out_dir}"
    echo "$sha" > "${out_dir}/sha256sum"

    # Detect hole
    first_line="$(head -n 1 "$source_file")"
    first_line="${first_line// /}"
    hole_prefix="#hole:"
    if [[ ! "$first_line" == "$hole_prefix"* ]]; then
        echo "Missing '# hole: ' line, skipping ${base}." >&2
        continue
    fi
    hole="${first_line##"$hole_prefix"}"

    # Run
    run_tmp_dir="./expected/${base}-orig"
    ../node_modules/.bin/golfc submit -h "$hole" -l assembly \
        -i "$source_file" --no-auth -o "${run_tmp_dir}"
    cp "${run_tmp_dir}"/{output,errors} "${out_dir}"
    rm -r "${run_tmp_dir}"
done
