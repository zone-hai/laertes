#!/bin/bash

rm libxml2_rust.txt
rm -r rewrite-workspace/libxml2_rust
cd test-inputs/libxml2_rust
cargo clean
cargo build
cd ../..

cp -r test-inputs/libxml2_rust rewrite-workspace/

# cp -r resolve-imports/libxml2_rust rewrite-workspace/

python3 libxml2-rust.py
export RUST_LOG=debug
RUST_BACKTRACE=1 cargo run --release --bin resolve-imports -- `cat rewrite-invocations/libxml2_rust` > libxml2_rust.txt
export RUST_LOG=debug
RUST_BACKTRACE=1 cargo run --release --bin resolve-lifetimes -- -f --merge-field-lifetimes `cat rewrite-invocations/libxml2_rust` >> libxml2_rust.txt
