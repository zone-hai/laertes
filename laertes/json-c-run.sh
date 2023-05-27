#!/bin/bash

rm jsonc_rust.txt
rm -r rewrite-workspace/jsonc_rust
cd test-inputs/jsonc_rust
cargo clean
cargo build
cd ../..

cp -r test-inputs/jsonc_rust rewrite-workspace/

# 注意更改json-c-rust.py中第16行的路径位置
python3 json-c-rust.py
export RUST_LOG=debug
RUST_BACKTRACE=1 cargo run --release --bin resolve-imports -- `cat rewrite-invocations/jsonc_rust` > jsonc_rust.txt
# export RUST_LOG=debug
# 可能的运行错误信息和输出，写入文档：jsonc_rust.txt
# cargo run --release --bin resolve-lifetimes -- -f --merge-field-lifetimes `cat rewrite-invocations/jsonc_rust` > jsonc_rust.txt