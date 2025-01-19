#!/bin/bash
set -euo pipefail

mkdir -p expected

# TODO remove old expected

trim_per_line() {
    sed -E 's/\s+$//g'
}

unescape() {
    sed "s/&#39;/'/g" < "$1" | sed "s/&nbsp;/ /g" | trim_per_line | sponge "$1";
}

add_trailing_newline_if_missing() {
    if [ -s "$1" ] && [ "$(tail -c1 "$1"; echo x)" != $'\nx' ]; then
        echo "" >>"$1"
    fi
}

# Turn sources/* into expected/* by running on code.golf servers
for source_file in sources/*.s; do
    base="${source_file##sources/}"
    base="${base%.s}"
    out_dir="./expected/${base}"

    # Check if it's already ran
    sha="$(sha256sum "$source_file")"
    sha_file="${out_dir}/sha256sum"
    old_sha="$(cat "$sha_file" 2>/dev/null || true)"
    if [[ "$old_sha" == "$sha" ]]; then
        continue
    fi

    # Stale or missing output
    echo "Updating ${base}."
    rm -r "${out_dir}" 2> /dev/null || true
    mkdir -p "${out_dir}"

    # Detect hole
    first_line="$(head -n 1 "$source_file")"
    first_line="${first_line// /}"
    hole_prefix="#hole:"
    if [[ ! "$first_line" == "$hole_prefix"* ]]; then
        echo "Missing '# hole: ' line, skipping ${base}." >&2
        continue
    fi
    hole="${first_line##"$hole_prefix"}"

    # Substitute macro
    real_asm_path="real_sources/$base.s"
    ./insert-macros.mjs "$source_file" "$real_asm_path"

    # Run
    run_tmp_dir="./expected/${base}-orig"
    ../../node_modules/.bin/golfc submit -h "$hole" -l assembly \
        -i "$real_asm_path" --no-auth -o "${run_tmp_dir}"
    # add trailing newline if missing.
    # shellcheck disable=SC1003
    sed -i '$ a\' "${run_tmp_dir}"/{output,errors}
    cp "${run_tmp_dir}"/{output,errors} "${out_dir}"
    rm -r "${run_tmp_dir}"

    unescape "$out_dir/errors"
    add_trailing_newline_if_missing "$out_dir/errors"

    # Only update SHA if successful
    echo "$sha" > "$sha_file"
done
