#!/bin/bash
rm curl-rust.txt
rm -r rewrite-workspace/curl-rust
cp -r test-inputs/curl-rust rewrite-workspace/
cd rewrite-workspace/curl-rust
cargo clean
cargo build
cd ../..

python3 curl-rust.py

# +++ resolve-imports +++
# save to ./after-resolve-imports
export RUST_LOG=debug
cargo run --release --bin resolve-imports -- `cat rewrite-invocations/curl-rust` >curl-imports.txt
cp -r rewrite-workspace/curl-rust after-resolve-imports/

rm curl-rust.txt
rm -r rewrite-workspace/curl-rust
cp -r after-resolve-imports/expand_bitfield/curl-rust rewrite-workspace/
cd rewrite-workspace/curl-rust
cargo clean
cargo build
cd ../..

# +++ resolve-lifetimes +++
# save to after-resolve-lifetimes
# export RUST_LOG=debug
cargo run --release --bin resolve-lifetimes -- -f --merge-field-lifetimes `cat rewrite-invocations/curl-rust` >> curl-rust.txt
