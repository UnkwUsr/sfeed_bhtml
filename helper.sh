#!/bin/env bash
set -euo pipefail
shopt -s failglob

cd ~/.sfeed/ || exit 1

sfeed_update
(cd ~/Projects/sfeed_html_full/ && cargo build)

mkdir -p prev_html
new_res="./prev_html/res_$(date +%s).html"
~/Projects/sfeed_html_full/target/debug/a ./feeds/* >> "$new_res"

# if res is not empty
if [ -s "$new_res" ]; then
    quteb "$(realpath "$new_res")"
else
    rm "$new_res"
fi
