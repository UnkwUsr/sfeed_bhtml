#!/bin/env bash
set -euo pipefail
shopt -s failglob

cd ~/.sfeed/ || exit 1

sfeed_update

mkdir -p generated_htmls
new_res="./generated_htmls/res_$(date +'%Y-%m-%d_%H:%M:%S').html"
# tac - revert lines order, so that feeds will be from oldest to newest
cat ./feeds/* | tac | sfeed_bhtml >> "$new_res"

# if file is not empty
if [ -s "$new_res" ]; then
    $BROWSER "$(realpath "$new_res")" & disown
else
    # remove redundant empty file
    rm "$new_res"
fi
