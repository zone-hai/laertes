#!/bin/bash
rm curl-rust.txt
# cp -r test-inputs/curl-rust rewrite-workspace/
rm -r rewrite-workspace/curl-rust
cd resolve-imports/curl-rust
cargo clean
cargo build
cd ../..
cp -r resolve-imports/curl-rust rewrite-workspace/

python3 curl-rust.py
export RUST_LOG=debug
# cargo run --release --bin resolve-imports -- `cat rewrite-invocations/curl-rust` >curl-imports.txt

RUST_BACKTRACE=1 cargo run --release --bin resolve-lifetimes -- -f --merge-field-lifetimes `cat rewrite-invocations/curl-rust` >> curl-rust.txt
