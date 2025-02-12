#! /bin/bash

cd rust
cargo build --release
cd ..
hyperfine --warmup 3 'php ./php/script.php' './rust/target/release/block-parser-rust-benchmark' -N 

