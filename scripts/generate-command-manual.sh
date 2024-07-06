#!/bin/bash

OUT_DIR="docs/commands.md"

for cmd in src/commands/*.rs; do
    cmd=$(basename "$cmd")
    [ "$cmd" == "mod.rs" ] && continue
    cmd=${cmd%*.rs}
    cmd=${cmd/_/-}
    
    echo "Running helpf for command $cmd ..."
    output=$(cargo run --quiet -- "$cmd" --help)

    headers+="- [$cmd](#$cmd): \`$(echo "$output" | head -n1)\`\n"
    mans+="### $cmd\n\n> $ goup help $cmd\n\n\`\`\`\n$output\n\`\`\`\n\n"
done

{
    echo -e "# Available Commands\n\n"
    echo -e "> $ goup help\n"
    echo -e "\`\`\`\n$(cargo run --quiet -- help)\n\`\`\`\n\n"
    echo -e "## Index\n\n"
    echo -e "$headers"
    echo -e "## Details\n\n"
    echo -e "$mans"
} > $OUT_DIR