#!/bin/env bash
set -euo pipefail
shopt -s failglob

cd ~/.sfeed/ || exit 1

sfeed_update
(cd ~/Projects/sfeed_html_full/ && cargo build)

mkdir -p generated_htmls
new_res="./generated_htmls/res_$(date +'%Y-%m-%d_%H:%M:%S').html"
cat ./feeds/* | ~/Projects/sfeed_html_full/target/debug/a >> "$new_res"

# if file is not empty
if [ -s "$new_res" ]; then
    quteb "$(realpath "$new_res")"
else
    # remove redundant empty file
    rm "$new_res"
fi
