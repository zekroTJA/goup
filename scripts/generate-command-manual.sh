#!/bin/bash

OUT_DIR="docs/commands.md"

header=""
mans=""

for cmd in src/commands/*.rs; do
    cmd=$(basename $cmd)
    [ "$cmd" == "mod.rs" ] && continue
    cmd=${cmd%*.rs}
    cmd=${cmd/_/-}
    
    output=$(cargo run --quiet -- "$cmd" --help)

    headers+="- [$cmd](#$cmd): \``echo "$output" | head -n1`\`\n"
    mans+="### $cmd\n\n> $ goup help $cmd\n\n\`\`\`\n$output\n\`\`\`\n\n"
done

echo -e "# Available Commands\n\n" > $OUT_DIR
echo -e "> $ goup help\n" >> $OUT_DIR
echo -e "\`\`\`\n`cargo run --quiet -- help`\n\`\`\`\n\n" >> $OUT_DIR
echo -e "## Index\n\n" >> $OUT_DIR
echo -e "$headers" >> $OUT_DIR
echo -e "## Details\n\n" >> $OUT_DIR
echo -e "$mans" >> $OUT_DIR