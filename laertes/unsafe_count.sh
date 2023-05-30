#!/bin/bash
rm ./unsafe_count.txt
RUST_BACKTRACE=1 cargo run --release --bin unsafe-counter -- `cat invocations/jsonc_rust` > unsafe_count.txt
# RUST_BACKTRACE=1 cargo run --release --bin unsafe-counter -- `cat invocations/libxml2_rust` >> unsafe_count.txt
# RUST_BACKTRACE=1 cargo run --release --bin unsafe-counter -- `cat invocations/curl-rust` >> unsafe_count.txt