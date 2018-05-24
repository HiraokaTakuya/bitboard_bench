#!/bin/bash

(cd rust_version; RUSTFLAGS="-C target-cpu=native" cargo build --release)
(cd c++_version; make)
sleep 2
echo
echo "rust_version"
./rust_version/target/release/rust_version
echo
echo "c++_version"
./c++_version/bench
