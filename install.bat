cargo build --release
@echo off
cd oslobanden-wasm
@echo on
wasm-pack build --target web --out-name frontend
@echo off
cd ..
cp target/release/oslobanden.exe bin/oslobanden.exe
cp oslobanden-wasm/pkg/frontend_bg.wasm bin/frontend_bg.wasm
cp oslobanden-wasm/pkg/frontend.js bin/frontend.js
cp oslobanden-wasm/html/index.html bin/index.html
