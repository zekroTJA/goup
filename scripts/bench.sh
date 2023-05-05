#!/bin/bash

cargo build -r

hyperfine \
    --runs 20 \
    --warmup 3 \
    --export-markdown "bench-lsr.md" \
    './target/release/goup lsr'