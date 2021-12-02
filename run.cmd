@echo off

cd "day%1%"
cargo run --manifest-path="part%2%/Cargo.toml"
cd ..
