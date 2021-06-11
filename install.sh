cargo build --release
### Use prebuilt wasm binary
# cd oslobanden-wasm
# wasm-pack build --target web --out-name frontend
# cd ..
systemctl stop oslobanden
cp target/release/oslobanden bin/oslobanden
systemctl start oslobanden
# cp oslobanden-wasm/pkg/frontend_bg.wasm bin/frontend_bg.wasm
# cp oslobanden-wasm/pkg/frontend.js bin/frontend.js
# cp oslobanden-wasm/html/index.html bin/index.html
