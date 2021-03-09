CARGO_TARGET_DIR=target cargo build --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/mywasm.wasm --typescript --out-dir pkg/