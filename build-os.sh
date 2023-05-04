#!/usr/bin/sh
cd os
cargo build --target-dir ../target
cd ..

cargo run --release --bin assembleos target/x86_64/debug/os