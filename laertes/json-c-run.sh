#!/bin/bash

rm jsonc_rust.txt
rm -r rewrite-workspace/jsonc_rust
cd test-inputs/jsonc_rust
cargo clean
cargo build
cd ../..

cp -r test-inputs/jsonc_rust rewrite-workspace/

export RUST_LOG=debug
RUST_BACKTRACE=1 cargo run --release --bin resolve-imports -- `cat rewrite-invocations/jsonc_rust` > jsonc_rust.txt
python3 json-c-rust.py
# export RUST_LOG=debug
# RUST_BACKTRACE=1 cargo run --release --bin resolve-lifetimes -- -f --merge-field-lifetimes `cat rewrite-invocations/jsonc_rust` > jsonc_rust.txt