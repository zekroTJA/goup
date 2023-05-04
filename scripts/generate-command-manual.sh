#!/bin/bash

OUT_DIR="docs/commands.md"

cat << EOF > $OUT_DIR
# Available Commands

EOF

header=""
mans=""

for cmd in src/commands/*.rs; do
    cmd=$(basename $cmd)
    [ "$cmd" == "mod.rs" ] && continue
    cmd=${cmd%*.rs}
    cmd=${cmd/_/-}
    
    output=$(cargo run --quiet -- "$cmd" --help)

    headers+="- [$cmd](#$cmd): \``echo "$output" | head -n1`\`\n"
    mans+="## $cmd\n\n\`\`\`\n$output\n\`\`\`\n\n"
done

echo -e "$headers" >> $OUT_DIR
echo -e "$mans" >> $OUT_DIR