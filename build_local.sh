#!/bin/bash
set -e

cd contract

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
cp target/wasm32-unknown-unknown/release/pool_details.wasm ./out/local.wasm
