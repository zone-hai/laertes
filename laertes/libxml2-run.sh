#!/bin/bash

rm libxml2_rust.txt
rm -r rewrite-workspace/libxml2_rust
cp -r test-inputs/libxml2_rust rewrite-workspace/
cd rewrite-workspace/libxml2_rust
cargo clean
cargo build
cd ../..

python3 libxml2-rust.py
# +++ resolve-imports +++
# save to ./after-resolve-imports
export RUST_LOG=debug
cargo run --release --bin resolve-imports -- `cat rewrite-invocations/libxml2_rust` > libxml2_rust.txt
cp -r rewrite-workspace/libxml2_rust after-resolve-imports/


# +++ resolve-lifetimes +++
# save to after-resolve-lifetimes
# export RUST_LOG=debug
# RUST_BACKTRACE=1 cargo run --release --bin resolve-lifetimes -- -f --merge-field-lifetimes `cat rewrite-invocations/libxml2_rust` >> libxml2_rust.txt
